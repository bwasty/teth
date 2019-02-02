//! Toy Ethereum client closely following the [Yellow Paper](https://github.com/ethereum/yellowpaper/).
//! Structs, fields and methods are annotated with their formal definition where applicable.

use tiny_keccak::keccak256;

/// σ
/// 
/// The world state (_state_), is a mapping between addresses (160-bit identifiers) and account states.
#[derive(Debug)]
pub struct WorldState {
    // TODO!!: this should probably be a hash mapping from address to state...
    pub accounts: Vec<AccountState>
}

#[allow(dead_code)]
impl WorldState {
    /// An account is dead when its account state is non-existent or empty:
    /// 
    /// DEAD(σ, a) ≡ σ[a] = ∅ ∨ EMPTY(σ, a)
    pub fn is_account_dead(&self, account: &AccountState) -> bool {
        account.is_empty() || !self.accounts.contains(account)
    }
}

pub type Hash = [u8; 32];
pub type Address = [u8; 20];

/// σ[a]
#[derive(Debug, PartialEq)]
pub struct AccountState {
    /// A scalar value equal to the number of transactions sent from this address or, 
    /// in the case of accounts with associated code, the number of contract-creations made by 
    /// this account. For account of address a in state σ, this would be formally denoted σ[a]<sub>n</sub>.
    pub nonce: u64,
    /// A scalar value equal to the number of Wei owned by this address. Formally denoted σ[a]<sub>b</sub>.
    pub balance: u64, 
    /// A 256-bit hash of the root node of a Merkle Patricia tree that encodes the storage contents 
    /// of the account (a mapping between 256-bit integer values), encoded into the trie as a mapping 
    /// from the Keccak 256-bit hash of the 256-bit integer keys to the RLP-encoded 256-bit integer 
    /// values. The hash is formally denoted σ[a]<sub>s</sub>.
    /// TODO?: It shall be understood that σ[a]<sub>s</sub> is not a ‘physical’ member of the account and does 
    /// not contribute to its later serialisation.
    pub storage_root: Hash,
    /// The hash of the EVM code of this account — this is the code that gets executed should this 
    /// address receive a message call; it is immutable and thus, unlike all other fields, cannot 
    /// be changed after construction. All such code fragments are contained in the state database 
    /// under their corresponding hashes for later retrieval. This hash is formally denoted σ[a]<sub>c</sub>, 
    /// and thus the code may be denoted as b, given that KEC(b) = σ[a]<sub>c</sub>.
    pub code_hash: Hash,
}

impl Default for AccountState {
    fn default() -> Self {
        AccountState {
            nonce: 0,
            balance: 0,
            storage_root: keccak256(&[]),
            code_hash: keccak256(&[]),
        }
    }
}

#[allow(dead_code)]
impl AccountState {
    /// If the codeHash field is the Keccak-256 hash of the empty string, i.e. σ[a]<sub>c</sub> = KEC(()), 
    /// then the node represents a simple account, sometimes referred to as a “non-contract” account.
    pub fn has_code(&self) -> bool {
        self.code_hash != keccak256(&[])
    }

    /// An account is empty when it has no code, zero nonce and zero balance:
    /// 
    /// EMPTY(σ, a) ≡ σ[a]<sub>c</sub> = KEC(()) ∧ σ[a]<sub>n</sub> = 0 ∧ σ[a]<sub>b</sub> = 0
    pub fn is_empty(&self) -> bool {
        self.nonce == 0 && self.balance == 0 && !self.has_code()
    }
}

// TODO!: world-state collapse function L_S
// This function, LS, is used alongside the trie function to provide a short identity (hash) of the world state

// TODO!: v is the account validity function...

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
    pub nonce: u64,
    /// A scalar value equal to the number of Wei to be paid per unit of _gas_ for all computation costs incurred 
    /// as a result of the execution of this transaction; formally T<sub>p</sub>.
    pub gas_price: u64,
    /// A scalar value equal to the maximum amount of gas that should be used in executing this transaction. 
    /// This is paid up-front, before any computation is done and may not be increased later; formally T<sub>g</sub>.
    pub gas_limit: u64,
    /// The 160-bit address of the message call’s recipient or, for a contract creation transaction, ∅, 
    /// used here to denote the only member of B<sub>0</sub>; formally T<sub>t</sub>.
    pub to: Option<Address>,
    // TODO!!!: continue...
    // pub value: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_state() {
        let mut acc = AccountState::default();
        assert!(!acc.has_code());
        assert!(acc.is_empty());

        acc.code_hash = keccak256(&[1, 2, 3]);
        assert!(acc.has_code());
        assert!(!acc.is_empty());
    }

    #[test]
    fn test_world_state() {
        let world = WorldState { accounts: vec![AccountState::default()] };
        assert!(world.is_account_dead(&AccountState::default()));
    }

    #[test]
    fn test_transaction() {
        let _t = Transaction::default();
    }
    
}
