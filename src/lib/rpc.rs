use jsonrpc_ws_server::jsonrpc_core::{IoHandler, Result};
use jsonrpc_ws_server::ServerBuilder;
use jsonrpc_derive::rpc;

use ethereum_types::Address;
use crate::lib::{Wei, WorldState, BlockChain};

#[rpc]
pub trait Rpc {
    #[rpc(name = "eth_getBalance")]
    fn get_balance(&self, address: Address, block: String) -> Result<Wei>;

    // TODO: limit, offset?
    #[rpc(name = "teth_topAccounts")]
    fn top_accounts(&self, limit: Option<usize>, offset: Option<usize>) -> Result<Vec<(Address, Wei)>>;
}

struct RpcImpl {
    world_state: WorldState,
    #[allow(dead_code)]
    block_chain: BlockChain,
}

impl RpcImpl {
    pub fn new(world_state: WorldState, block_chain: BlockChain) -> Self {
        Self {
            world_state,
            block_chain,
        }
    }
}
impl Rpc for RpcImpl {
    fn get_balance(&self, address: Address, _block: String) -> Result<Wei> {
        let account = self.world_state.accounts.get(&address);
        if let Some(account) = account {
            Ok(account.balance)
        } else {
            Ok(0.into())
        }
    }

    fn top_accounts(&self, offset: Option<usize>, limit: Option<usize>) -> Result<Vec<(Address, Wei)>> {
        let mut balances: Vec<(Address, Wei)> = self.world_state.accounts.iter()
            .map(|(address, account)| (*address, account.balance))
            .collect();
        balances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // desc by balance

        let limit = limit.unwrap_or(5);
        let offset = offset.unwrap_or(0);

        Ok(balances[offset..offset+limit].to_vec())
    }
}

pub fn start_server(world_state: WorldState, block_chain: BlockChain) {
    let mut io = IoHandler::new();
    let rpc = RpcImpl::new(world_state, block_chain);
    io.extend_with(rpc.to_delegate());

    let server = ServerBuilder::new(io)
        .start(&"0.0.0.0:8546".parse().unwrap())
        .expect("Server must start with no issues");

    server.wait().unwrap()
}
