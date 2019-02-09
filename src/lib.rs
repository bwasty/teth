//! Toy Ethereum client closely following the [Yellow Paper](https://github.com/ethereum/yellowpaper/) ([PDF](https://ethereum.github.io/yellowpaper/paper.pdf)).
//! Structs, fields and methods are annotated with their formal definition where applicable.

use std::collections::HashMap;
use std::time::SystemTime;

use ethereum_types::{Address, Bloom, H256, U256};
use lazy_static::lazy_static;
use rlp::{Encodable, RlpStream, EMPTY_LIST_RLP};
use tiny_keccak::keccak256;

/// σ
///
/// The world state (_state_), is a mapping between addresses (160-bit identifiers) and account states.
#[derive(Debug)]
pub struct WorldState {
    pub accounts: HashMap<Address, AccountState>,
}

#[allow(dead_code)]
impl WorldState {
    /// An account is dead when its account state is non-existent or empty:
    ///
    /// DEAD(σ, a) ≡ σ[a] = ∅ ∨ EMPTY(σ, a)
    pub fn is_account_dead(&self, account: &AccountState) -> bool {
        account.is_empty() || !self.accounts.values().any(|x| x == account)
    }
}

/// Alias for `ethereum_types::U256`
pub type Wei = U256;

/// σ[a]
#[derive(Debug, PartialEq, Clone)]
pub struct AccountState {
    /// A scalar value equal to the number of transactions sent from this address or,
    /// in the case of accounts with associated code, the number of contract-creations made by
    /// this account. For account of address a in state σ, this would be formally denoted σ[a]<sub>n</sub>.
    pub nonce: U256,
    /// A scalar value equal to the number of Wei owned by this address. Formally denoted σ[a]<sub>b</sub>.
    pub balance: Wei,
    /// A 256-bit hash of the root node of a Merkle Patricia tree that encodes the storage contents
    /// of the account (a mapping between 256-bit integer values), encoded into the trie as a mapping
    /// from the Keccak 256-bit hash of the 256-bit integer keys to the RLP-encoded 256-bit integer
    /// values. The hash is formally denoted σ[a]<sub>s</sub>.
    /// TODO?: It shall be understood that σ[a]<sub>s</sub> is not a ‘physical’ member of the account and does
    /// not contribute to its later serialisation.
    pub storage_root: H256,
    /// The hash of the EVM code of this account — this is the code that gets executed should this
    /// address receive a message call; it is immutable and thus, unlike all other fields, cannot
    /// be changed after construction. All such code fragments are contained in the state database
    /// under their corresponding hashes for later retrieval. This hash is formally denoted σ[a]<sub>c</sub>,
    /// and thus the code may be denoted as b, given that KEC(b) = σ[a]<sub>c</sub>.
    pub code_hash: H256,
}

impl Default for AccountState {
    fn default() -> Self {
        AccountState {
            nonce: 0.into(),
            balance: 0.into(),
            storage_root: keccak256(&[]).into(),
            code_hash: keccak256(&[]).into(),
        }
    }
}

#[allow(dead_code)]
impl AccountState {
    /// If the codeHash field is the Keccak-256 hash of the empty string, i.e. σ[a]<sub>c</sub> = KEC(()),
    /// then the node represents a simple account, sometimes referred to as a “non-contract” account.
    pub fn has_code(&self) -> bool {
        self.code_hash != keccak256(&[]).into()
    }

    /// An account is empty when it has no code, zero nonce and zero balance:
    ///
    /// EMPTY(σ, a) ≡ σ[a]<sub>c</sub> = KEC(()) ∧ σ[a]<sub>n</sub> = 0 ∧ σ[a]<sub>b</sub> = 0
    pub fn is_empty(&self) -> bool {
        self.nonce.is_zero() && self.balance.is_zero() && !self.has_code()
    }
}

/// p(a) ≡ KEC(a), RLP(σ[a]n, σ[a]b, σ[a]s, σ[a]c))
impl Encodable for AccountState {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.balance);
        s.append(&self.storage_root);
        s.append(&self.code_hash);
    }
}

// TODO!: world-state collapse function L_S
// This function, LS, is used alongside the trie function to provide a short identity (hash) of the world state

// TODO!: v is the account validity function...

/// ECDSA signature
#[derive(Debug, Default)]
pub struct Signature {
    pub v: u8,
    pub r: U256,
    pub s: U256,
}

lazy_static! {
    static ref SECP256K1N: U256 = U256::from_dec_str(
        "115792089237316195423570985008687907852837564279074904382605163141518161494337"
    )
    .unwrap();
}

#[allow(dead_code)]
impl Signature {
    pub fn new(v: u8, r: U256, s: U256) -> Self {
        Self { v, r, s }
    }

    /// We declare that an ECDSA signature is invalid unless all the following conditions are true:  
    ///     0 < r < secp256k1n  
    ///     0 < s < secp256k1n÷2+1  
    ///     v ∈ {27, 28}  
    /// where:  
    ///     secp256k1n = 115792089237316195423570985008687907852837564279074904382605163141518161494337
    pub fn is_valid(&self) -> bool {
        self.r > U256::zero()
            && self.r < *SECP256K1N
            && self.s > U256::zero()
            && self.s < *SECP256K1N / 2 + 1
            && self.v == 27
            || self.v == 28
    }
}

/// T
///
/// A transaction (formally, T) is a single cryptographically-signed instruction constructed by an
/// actor externally to the scope of Ethereum. While it is assumed that the ultimate external actor
/// will be human in nature, software tools will be used in its construction and dissemination.
/// There are two types of transactions: those which result in message calls and those which
/// result in the creation of new accounts with associated code (known informally as ‘contract
/// creation’).
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Transaction {
    /// A scalar value equal to the number of transactions sent by the sender; formally T<sub>n</sub>.
    pub nonce: U256,
    /// A scalar value equal to the number of Wei to be paid per unit of _gas_ for all computation costs incurred
    /// as a result of the execution of this transaction; formally T<sub>p</sub>.
    pub gas_price: Wei,
    /// A scalar value equal to the maximum amount of gas that should be used in executing this transaction.
    /// This is paid up-front, before any computation is done and may not be increased later; formally T<sub>g</sub>.
    pub gas_limit: U256,
    /// The 160-bit address of the message call’s recipient or, for a contract creation transaction, ∅,
    /// used here to denote the only member of B<sub>0</sub>; formally T<sub>t</sub>.
    pub to: Option<Address>,
    /// A scalar value equal to the number of Wei to be transferred to the message call’s recipient or,
    /// in the case of contract creation, as an endowment to the newly created account; formally T<sub>v</sub>.
    pub value: Wei,

    /// Values corresponding to the signature of the transaction and used to determine the sender of the transaction;
    /// formally T<sub>w</sub>, T<sub>r</sub> and T<sub>s</sub>. This is expanded in Appendix F.
    pub signature: Signature,

    /// An unlimited size byte array specifying the EVM-code for the account initialisation procedure, formally
    /// T<sub>i</sub>.  
    /// `init` is an EVM-code fragment; it returns the `body`, a second fragment of code that executes each time the
    /// account receives a message call (either through a transaction or due to the internal execution of code).
    /// `init` is executed only once at account creation and gets discarded immediately thereafter.  
    /// Empty for message calls.
    pub init: Option<Vec<u8>>,

    /// An unlimited size byte array specifying the input data of the message call, formally T<sub>d</sub>.  
    /// Empty for contract creation transactions.
    pub data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Transaction {
    /// S
    ///
    /// Appendix F specifies the function, S, which maps transactions to the sender, and happens through
    /// the ECDSA of the SECP-256k1 curve, using the hash of the transaction (excepting the latter three
    /// signature fields) as the datum to sign. For the present we simply assert that the sender of a
    /// given transaction T can be represented with S(T).
    pub fn sender(&self) -> Address {
        unimplemented!() // TODO!
    }
}

/// H
#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct BlockHeader {
    /// The Keccak 256-bit hash of the parent block’s header, in its entirety; formally H<sub>p</sub>.
    pub parent_hash: H256,
    /// The Keccak 256-bit hash of the ommers list portion of this block; formally H<sub>o</sub>.
    pub ommers_hash: H256,
    /// The 160-bit address to which all fees collected from the successful mining of this block
    /// be transferred; formally H<sub>c</sub>.
    pub beneficiary: Address,
    /// The Keccak 256-bit hash of the root node of the state trie, after all transactions are
    /// executed and finalisations applied; formally H<sub>r</sub>.
    pub state_root: H256,
    /// The Keccak 256-bit hash of the root node of the trie structure populated with each
    /// transaction in the transactions list portion of the block; formally H<sub>t</sub>.
    pub transactions_root: H256,
    /// The Keccak 256-bit hash of the root node of the trie structure populated with the receipts
    /// of each transaction in the transactions list portion of the block; formally H<sub>e</sub>.
    pub receipts_root: H256,
    /// The Bloom filter composed from indexable information (logger address and log topics) contained
    /// in each log entry from the receipt of each transaction in the transactions list; formally H<sub>b</sub>.
    pub logs_bloom: Bloom,
    /// A scalar value corresponding to the difficulty level of this block. This can be calculated from
    /// the previous block’s difficulty level and the timestamp; formally H<sub>d</sub>.
    pub difficulty: U256,
    /// A scalar value equal to the number of ancestor blocks.
    /// The genesis block has a number of zero; formally H<sub>i</sub>.
    pub number: u64,
    /// A scalar value equal to the current limit of gas expenditure per block; formally H<sub>l</sub>.
    pub gas_limit: U256,
    /// A scalar value equal to the total gas used in transactions in this block; formally H<sub>g</sub>.
    pub gas_used: U256,
    /// A scalar value equal to the reasonable output of Unix’s time() at this block’s inception;
    /// formally H<sub>s</sub>.
    pub timestamp: u64,
    /// An arbitrary byte array containing data relevant to this block.
    /// This must be 32 bytes or fewer; formally H<sub>x</sub>.
    pub extra_data: [u8; 32],
    /// A 256-bit hash which, combined with the nonce, proves that a sufficient amount of computation has
    /// been carried out on this block; formally H<sub>m</sub>.
    pub mix_hash: H256,
    /// A 64-bit value which, combined with the mix-hash, proves that a sufficient amount of computation has
    /// been carried out on this block; formally H<sub>n</sub>.
    pub nonce: u64,
}

/// B
///
/// The block in Ethereum is the collection of relevant pieces of information (known as the block header), _H_,
/// together with information corresponding to the comprised transactions, *T*, and a set of other block headers *U*
/// that are known to have a parent equal to the present block’s parent’s parent (such blocks are known as _ommers_).
///
/// Formally, we can refer to a block B:  
/// B ≡ (B<sub>H</sub>, B<sub>T</sub>, B<sub>U</sub>)
#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Block {
    /// B<sub>H</sub>
    pub header: BlockHeader,
    /// B<sub>T</sub>
    pub transactions: Vec<Transaction>,
    /// B<sub>U</sub>  
    /// Also known as 'uncles'.
    pub ommers: Vec<BlockHeader>,
}

#[allow(dead_code)]
impl Block {
    /// Appendix I. Genesis Block
    ///
    /// The genesis block is 15 items, and is specified thus:  
    /// ((0<sub>256</sub>, KEC(RLP()), 0<sub>160</sub> , _stateRoot_, 0, 0, 0<sub>2048</sub> , 2<sup>17</sup> , 0, 0, 3141592, _time_, 0, 0<sub>256</sub> , KEC((42)), (), ())
    ///
    /// Where 0<sub>256</sub> refers to the parent hash, a 256-bit hash which is all zeroes;
    /// 0<sub>160</sub> refers to the beneficiary address, a 160-bit hash which is all zeroes;
    /// 0<sub>2048</sub> refers to the log bloom, 2048-bit of all zeros; 2<sup>17</sup> refers
    /// to the difficulty; the transaction trie root, receipt trie root, gas used, block number
    /// and extradata are both 0, being equivalent to the empty byte array. The sequences of both
    /// ommers and transactions are empty and represented by (). KEC((42)) refers to the Keccak
    /// hash of a byte array of length one whose first and only byte is of value 42, used for
    /// the nonce. KEC(RLP()) value refers to the hash of the ommer list in RLP, both empty lists.
    ///
    /// The proof-of-concept series include a development premine, making the state root hash some
    /// value _stateRoot_. Also _time_ will be set to the initial timestamp of the genesis block. The
    /// latest documentation should be consulted for those values.
    pub fn genesis_block() -> Self {
        Block {
            header: BlockHeader {
                parent_hash: H256::zero(),
                ommers_hash: keccak256(&EMPTY_LIST_RLP).into(),
                beneficiary: Address::zero(),
                state_root: H256::zero(), // TODO!: ??
                transactions_root: H256::zero(),
                receipts_root: H256::zero(),
                logs_bloom: Bloom::zero(),
                difficulty: (2 << 17).into(),
                number: 0,
                gas_limit: 3_141_592.into(),
                gas_used: 0.into(),
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                extra_data: [0; 32],
                mix_hash: H256::zero(),
                nonce: 42, // TODO: spec says KEC((42)), but it's a u64...?
            },
            transactions: vec![],
            ommers: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_state() {
        let mut acc = AccountState::default();
        assert!(!acc.has_code());
        assert!(acc.is_empty());

        acc.code_hash = keccak256(&[1, 2, 3]).into();
        assert!(acc.has_code());
        assert!(!acc.is_empty());
    }

    #[test]
    fn test_world_state() {
        let empty_acc = AccountState::default();
        let non_empty_acc = AccountState {
            balance: 1.into(),
            ..Default::default()
        };
        let non_empty_but_dead_acc = AccountState {
            balance: 2.into(),
            ..Default::default()
        };
        let world = WorldState {
            accounts: vec![
                (Address::random(), empty_acc.clone()),
                (Address::random(), non_empty_acc.clone()),
            ]
            .into_iter()
            .collect(),
        };
        assert!(world.is_account_dead(&empty_acc));
        assert!(!world.is_account_dead(&non_empty_acc));
        assert!(world.is_account_dead(&non_empty_but_dead_acc));
    }

    #[test]
    #[ignore] // TODO!!: missing implementation...
    fn test_transaction() {
        let t = Transaction::default();
        assert_eq!(t.sender(), Address::zero());
    }

    #[test]
    fn test_block_header() {
        let _h = BlockHeader::default();
    }

    #[test]
    fn test_block() {
        let _b = Block::default();
    }

    #[test]
    fn test_signature() {
        let sig = Signature::new;
        let s = sig(0, 0.into(), 0.into());
        assert!(!s.is_valid());

        let s1 = sig(27, 1.into(), 1.into());
        assert!(s1.is_valid())
    }
}
