//! Toy Ethereum client closely following the [Yellow Paper](https://github.com/ethereum/yellowpaper/) ([PDF](https://ethereum.github.io/yellowpaper/paper.pdf)).
//! Structs, fields and methods are annotated with their formal definition where applicable.
//! 
//! Currently implements a small subset of the [JSON-RPC API](https://github.com/ethereum/wiki/wiki/JSON-RPC) 
//! and a few custom methods. See the `rpc` module for details.
//! 
//! See [README.md](https://github.com/bwasty/teth/blob/master/README.md) for more info.

use structopt::StructOpt;

mod lib;
pub use lib::*;

fn main() {
    let opt = Options::from_args();
    match opt.commands {
        SubCommand::Node { bootstrap } => {
            if bootstrap {
                let state = WorldState::genesis_state();
                let mut block_chain = BlockChain::new();
                block_chain.add_block(Block::genesis_block());
                block_chain.add_block(Block::exodus_block());
                rpc::start_websocket_server(state, block_chain);
            } else {
                // TODO!: connect to master node (teth.malkut.net / localhost -> arg...)
                unimplemented!()
            }
        }
    }
}
