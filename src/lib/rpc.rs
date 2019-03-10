use std::sync::Mutex;

use ethereum_types::{Address, Bloom, H256, U256};
use jsonrpc_derive::rpc;
use jsonrpc_ws_server::jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_ws_server::ServerBuilder;
use serde::Serialize;

use crate::lib::{
    AccountState, Block, BlockChain, BlockHeader, Transaction, Wei, WorldState, ONE_ETHER,
};

/// Source: https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_getblockbyhash
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockResponse {
    /// The block number. `None` when it's a pending block.
    pub number: Option<u64>,
    pub hash: Option<H256>,
    pub parent_hash: H256,
    /// Hash of the generated proof-of-work. `None` when its pending block
    pub nonce: Option<u64>,
    // /// SHA3 of the uncles data in the block.
    // pub sha3_uncles: H256,
    /// The bloom filter for the logs of the block. `None` when its pending block.
    pub logs_bloom: Option<Bloom>,
    /// The root of the transaction trie of the block.
    pub transactions_root: H256,
    /// The root of the receipts trie of the block.
    pub receipts_root: H256,
    /// The address of the beneficiary to whom the mining rewards were given.
    pub miner: Address,
    pub difficulty: U256,
    pub total_difficulty: U256,
    pub extra_data: [u8; 32],
    /// The size of this block in bytes
    pub size: usize,
    pub gas_limit: U256,
    pub gas_used: U256,
    pub timestamp: u64,
    #[serde(rename = "transactions", skip_serializing_if = "Vec::is_empty")]
    pub full_transactions: Vec<TransactionResponse>,
    #[serde(rename = "transactions", skip_serializing_if = "Vec::is_empty")]
    pub transaction_hashes: Vec<H256>,
    /// Array of uncle hashes
    pub uncles: Vec<H256>,
}

impl BlockResponse {
    fn new(block: &Block, block_chain: &BlockChain, include_full_transactions: bool) -> Self {
        let header = &block.header;
        let mut response = Self {
            number: Some(header.number),
            hash: Some(header.hash()),
            parent_hash: header.parent_hash,
            nonce: Some(header.nonce),
            // sha3_uncles: keccak256(&encode(&block.ommers)).into(),
            logs_bloom: Some(header.logs_bloom), // TODO!: None for pending block..
            transactions_root: header.transactions_root,
            receipts_root: header.receipts_root,
            miner: header.beneficiary,
            difficulty: header.difficulty,
            total_difficulty: block_chain.total_difficulty(&block_chain.latest_block_hash),
            extra_data: header.extra_data,
            size: block.to_rlp().len(),
            gas_limit: header.gas_limit,
            gas_used: header.gas_used,
            timestamp: header.timestamp,
            full_transactions: vec![],
            transaction_hashes: vec![],
            uncles: block.ommers.iter().map(BlockHeader::hash).collect(),
        };
        if include_full_transactions {
            response.full_transactions = block
                .transactions
                .iter()
                .map(|t| TransactionResponse::new(t, Some(block)))
                .collect();
        } else {
            response.transaction_hashes = block.transactions.iter().map(|t| t.hash()).collect();
        }
        response
    }
}

/// Source: https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_gettransactionbyhash
#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    /// Hash of the block where this transaction was in. `None` when it's pending.
    pub block_hash: Option<H256>,
    /// Block number where this transaction was in. `None` when it's pending.
    pub number: Option<u64>,
    /// Address of the sender.
    pub from: Address,
    /// Gas provided by the sender.
    pub gas: Wei,
    /// Gas price provided by the sender in Wei
    pub gas_price: Wei,
    /// Hash of the transaction.
    pub hash: H256,
    /// The data sent along with the transaction.
    pub input: Option<Vec<u8>>,
    /// The number of transactions made by the sender prior to this one.
    pub nonce: U256,
    /// Address of the receiver. `None` when it's a contract creation transaction.
    pub to: Option<Address>,
    /// Integer of the transaction's index position in the block. `None` when it's pending.
    pub transaction_index: Option<usize>,
    /// Value transferred in Wei.
    pub value: Wei,
    /// ECDSA recovery id
    pub v: u8,
    /// ECDSA signature r
    pub r: U256,
    /// ECDSA signature s
    pub s: U256,
}

impl TransactionResponse {
    pub fn new(transaction: &Transaction, block: Option<&Block>) -> Self {
        let mut response = Self {
            from: transaction.sender(),
            gas: transaction.gas_limit,
            gas_price: transaction.gas_price,
            hash: transaction.hash(),
            input: transaction.data.clone(),
            nonce: transaction.nonce,
            to: transaction.to,
            value: transaction.value,
            v: transaction.signature.v,
            r: transaction.signature.r,
            s: transaction.signature.s,
            ..Self::default()
        };

        if let Some(block) = block {
            let header = &block.header;
            response.block_hash = Some(header.hash());
            response.number = Some(header.number);
            // TODO!: optimize?
            response.transaction_index = block
                .transactions
                .iter()
                .position(|t| t.hash() == transaction.hash())
        }

        response
    }
}

/// Trait for RPC methods, using jsonrpc-derive.
#[rpc]
pub trait Rpc {
    /// Returns the balance of the account of given address. See also
    /// [eth_getBalance](https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_getbalance).
    #[rpc(name = "eth_getBalance")]
    fn get_balance(&self, address: Address, block_number: String) -> Result<Wei>;

    /// Returns information about a block by block number. See also
    /// [eth_getBlockByNumber](https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_getblockbynumber).
    #[rpc(name = "eth_getBlockByNumber")]
    fn get_block_by_number(
        &self,
        number: String,
        return_transaction_objects: bool,
    ) -> Result<BlockResponse>;

    /// Non-standard RPC method to the 'top' accounts by balance. 
    #[rpc(name = "teth_topAccounts")]
    fn top_accounts(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<(Address, Wei)>>;

    /// Non-standard RPC method to get one Ether. 
    /// Works only when the account has no balance.
    #[rpc(name = "teth_faucet")]
    fn faucet(&self, address: Address) -> Result<bool>;

    #[rpc(name = "eth_syncing")]
    fn syncing(&self) -> Result<bool>;

    // #[rpc(name = "eth_subscribe")]
    // fn subscribe(&self, type_: String) -> Result<()>;
}

/// See `Rpc` trait for method descriptions.
pub struct RpcImpl {
    world_state: Mutex<WorldState>,
    block_chain: BlockChain,
}

impl RpcImpl {
    pub fn new(world_state: WorldState, block_chain: BlockChain) -> Self {
        Self {
            world_state: Mutex::new(world_state),
            block_chain,
        }
    }
}
impl Rpc for RpcImpl {
    fn get_balance(&self, address: Address, _block: String) -> Result<Wei> {
        let state = self.world_state.lock().unwrap();
        let account = state.accounts.get(&address);
        if let Some(account) = account {
            Ok(account.balance)
        } else {
            Ok(0.into())
        }
    }

    fn get_block_by_number(
        &self,
        number: String,
        return_transaction_objects: bool,
    ) -> Result<BlockResponse> {
        match number.as_ref() {
            "earliest" => Err(Error::internal_error()), // not implemented yet
            "latest" => {
                let block = &self.block_chain.blocks[&self.block_chain.latest_block_hash];
                Ok(BlockResponse::new(
                    &block,
                    &self.block_chain,
                    return_transaction_objects,
                ))
            }
            "pending" => Err(Error::internal_error()), // not implemented yet,
            _ => Err(Error::internal_error()),         // not implemented yet
        }
        // let block = self.block_chain.blocks.values()
        //     .find(|b| b.number)
    }

    fn top_accounts(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<(Address, Wei)>> {
        let state = self.world_state.lock().unwrap();
        let mut balances: Vec<(Address, Wei)> = state
            .accounts
            .iter()
            .map(|(address, account)| (*address, account.balance))
            .collect();
        balances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // desc by balance

        let limit = limit.unwrap_or(5);
        let offset = offset.unwrap_or(0);

        Ok(balances[offset..offset + limit].to_vec())
    }

    fn faucet(&self, address: Address) -> Result<bool> {
        // TODO!: require signed message
        let mut state = self.world_state.lock().unwrap();
        let mut account = state
            .accounts
            .get(&address)
            .cloned()
            .unwrap_or_else(AccountState::default);

        if account.balance == 0.into() {
            // TODO!: make it a transaction...
            account.balance = *ONE_ETHER;
            state.accounts.insert(address, account);
            Ok(true)
        } else {
            Err(Error::invalid_params("Account must be empty"))
        }
    }

    fn syncing(&self) -> Result<bool> {
        Ok(false)
    }

    // fn subscribe(&self, _type: String) -> Result<()> {
    //     Err(Error::invalid_request())
    // }
}

pub fn start_websocket_server(world_state: WorldState, block_chain: BlockChain) {
    let mut io = IoHandler::new();
    let rpc = RpcImpl::new(world_state, block_chain);
    io.extend_with(rpc.to_delegate());

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:8546".parse().unwrap())
        .expect("Server must start with no issues");

    server.wait().unwrap()
}
