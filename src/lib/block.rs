use ethereum_types::{Address, Bloom, H256, U256};
use rlp::EMPTY_LIST_RLP;
use tiny_keccak::keccak256;

use std::time::SystemTime;

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
    fn test_block_header() {
        let _h = BlockHeader::default();
    }

    #[test]
    fn test_block() {
        let _b = Block::default();
    }
}
