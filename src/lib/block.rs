use std::collections::HashMap;
use std::time::SystemTime;

use ethereum_types::{Address, Bloom, H256, U256};
use rlp::{encode, Encodable, RlpStream, EMPTY_LIST_RLP};
use tiny_keccak::keccak256;

use crate::lib::Transaction;

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

impl BlockHeader {
    // TODO!: refactor parameters... (self?)
    /// D(H) - Section 4.3.4, Equations 41-46
    pub fn difficulty(
        parent_number: u64,
        parent_difficulty: U256,
        num_parent_ommers: u64,
        parent_timestamp: u64,
        timestamp: u64,
        number: u64,
    ) -> U256 {
        let d0 = U256::from(131_072); // D<sub>0</sub>
        if parent_number == 0 {
            d0
        } else {
            // TODO: 'unsafe' casts below?
            let x = parent_difficulty / 2048;
            let y: i64 = if num_parent_ommers == 0 { 1 } else { 2 };
            let varsigma2 = (y - ((timestamp - parent_timestamp) / 9) as i64).max(-99);
            // H<sub>i</sub>'
            let fake_block_number = (number - 3_000_000).max(0) as u32;
            let epsilon = 2u64.pow(fake_block_number / 100_000 - 2);
            d0.max(parent_difficulty + x * varsigma2 + epsilon)
        }
    }

    /// The canonical gas limit H<sub>l</sub> of a block of header H must fulfil this relation (Equation 47).
    pub fn validate_gas_limit(gas_limit: U256, parent_gas_limit: U256) -> bool {
        gas_limit < parent_gas_limit + parent_gas_limit / 1024
            && gas_limit > parent_gas_limit - parent_gas_limit / 1024
            && gas_limit >= 5000.into()
    }

    /// The given **gasUsed must** correspond faithfully to the transactions listed:
    /// B<sub>H<sub>g</sub></sub>, the total gas used in the block, must be equal to the
    /// accumulated gas used according to the final transaction.
    /// (Section 11.2, Equation 158)
    pub fn validate_gas_used(&self) -> bool {
        unimplemented!() // TODO!
    }

    /// Equation 48
    pub fn validate_timestamp(timestamp: u64, parent_timestamp: u64) -> bool {
        timestamp > parent_timestamp
    }

    /// Equation 49
    pub fn validate_nonce(nonce: u64, difficulty: U256) -> bool {
        let _ = nonce <= (U256::from(2).pow(256.into()) / difficulty).as_u64();
        // TODO!: mix hash, PoW part...
        unimplemented!()
    }

    /// Block header validity function V(H) (Equation 50)
    pub fn validate(&self, parent: &BlockHeader) -> bool {
        // TODO!!: log each step if false
        let mut valid = Self::validate_nonce(self.nonce, self.difficulty);

        valid &= self.gas_used <= self.gas_limit;
        valid &= Self::validate_gas_limit(self.gas_limit, parent.gas_limit);
        // TODO!!: validate_gas_used? it's in a different equation...
        valid &= Self::validate_timestamp(self.timestamp, parent.timestamp);
        valid &= self.number == parent.number + 1;

        // no need to check: ∥Hx∥ ≤ 32 (extra_data is a [u8; 32])

        valid
    }

    pub fn to_rlp(&self) -> Vec<u8> {
        encode(self)
    }

    /// Keccak 256-bit hash
    pub fn hash(&self) -> H256 {
        keccak256(&self.to_rlp()).into()
    }

    // TODO!: impl decode...
    // pub fn from_rlp(data: &[u8]) -> Self {
    //     decode(data).expect("could not decode")
    // }
}

impl Encodable for BlockHeader {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.parent_hash);
        s.append(&self.ommers_hash);
        s.append(&self.beneficiary);
        s.append(&self.state_root);
        s.append(&self.transactions_root);
        s.append(&self.receipts_root);
        s.append(&self.logs_bloom);
        s.append(&self.difficulty);
        s.append(&self.number);
        s.append(&self.gas_limit);
        s.append(&self.gas_used);
        s.append(&self.timestamp);
        let extra_data: &[u8] = &self.extra_data;
        s.append(&extra_data);
        s.append(&self.mix_hash);
        s.append(&self.nonce);
    }
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

    /// Test block with transactions and genesis block as parent
    pub fn exodus_block() -> Self {
        let genesis = Self::genesis_block();
        let transaction = Transaction {
            nonce: 1.into(),
            gas_price: 2.into(),
            gas_limit: 84_000.into(),
            to: Some(Address::random()),
            value: 42_000.into(),
            data: Some(vec![]),
            ..Transaction::default()
        };
        let gas_used = transaction.intrinsic_gas();
        Block {
            header: BlockHeader {
                parent_hash: genesis.header.hash(),
                ommers_hash: keccak256(&EMPTY_LIST_RLP).into(),
                beneficiary: Address::zero(),
                state_root: H256::zero(),
                transactions_root: H256::zero(),
                receipts_root: H256::zero(),
                logs_bloom: Bloom::zero(),
                difficulty: (2 << 17).into(),
                number: 1,
                gas_limit: 3_141_592.into(),
                gas_used: gas_used.into(),
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                extra_data: [0; 32],
                mix_hash: H256::zero(),
                nonce: 42,
            },
            transactions: vec![transaction],
            ommers: vec![],
        }
    }

    pub fn to_rlp(&self) -> Vec<u8> {
        encode(self)
    }

    /// Keccak 256-bit hash
    pub fn hash(&self) -> H256 {
        keccak256(&self.to_rlp()).into()
    }
}

impl Encodable for Block {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.header);
        for t in &self.transactions {
            s.append(t);
        }
        for o in &self.ommers {
            s.append(o);
        }
    }
}

#[derive(Default)]
pub struct BlockChain {
    // TODO!: change value to RLP-encoded block? or only header?
    /// key: Keccak Hash of BlockHeader
    pub blocks: HashMap<H256, Block>,
    pub latest_block_hash: H256,
}

#[allow(dead_code)]
impl BlockChain {
    pub fn new() -> Self {
        let genesis_block = Block::genesis_block();
        let genesis_hash = genesis_block.header.hash();
        let mut blocks = HashMap::new();
        blocks.insert(genesis_hash, genesis_block);
        Self {
            blocks,
            latest_block_hash: genesis_hash,
        }
    }

    pub fn add_block(&mut self, block: Block) {
        let hash = block.header.hash();
        self.blocks.insert(hash, block);
        self.latest_block_hash = hash;
    }

    /// Section 10, Equation 153, 154
    pub fn total_difficulty(&self, block_hash: &H256) -> U256 {
        let mut block = &self.blocks[block_hash];
        let mut total_difficulty = block.header.difficulty;
        while block.header.parent_hash != H256::zero() {
            block = &self.blocks[&block.header.parent_hash];
            total_difficulty += block.header.difficulty;
        }
        total_difficulty
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_header() {
        let _h = BlockHeader::default();
    }

    #[test]
    fn test_block_header_difficulty() {
        let diff = BlockHeader::difficulty;
        let d0 = diff(0, 0.into(), 0, 0, 0, 0);
        assert_eq!(d0, U256::from(131_072));

        // TODO!: test for non-zero case(s)
    }

    #[test]
    fn test_block() {
        let _b = Block::default();
    }

    #[test]
    fn test_blockchain_total_difficulty() {
        let mut block_chain = BlockChain::new();
        assert_eq!(block_chain.total_difficulty(), (2 << 17).into());
        let mut new_block = Block::genesis_block();
        new_block.header.parent_hash = block_chain.latest_block_hash;
        block_chain.add_block(new_block);
        assert_eq!(block_chain.total_difficulty(), ((2 << 17) * 2).into());
    }

    #[test]
    fn print_blockheader_sizes() {
        // run with `cargo test -- --nocapture size --test-threads=1`
        println!("\nBlockHeader sizes:");
        println!(
            "struct:                   {:>3} bytes",
            std::mem::size_of::<BlockHeader>()
        );
        let mut blockheader = BlockHeader::default();
        let rlp: Vec<u8> = rlp::encode(&blockheader);
        println!("rlp default:              {:>3} bytes", rlp.len());
    }
}
