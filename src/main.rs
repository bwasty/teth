//! Toy Ethereum client closely following the [Yellow Paper](https://github.com/ethereum/yellowpaper/) ([PDF](https://ethereum.github.io/yellowpaper/paper.pdf)).
//! Structs, fields and methods are annotated with their formal definition where applicable.
//! See [README.md](https://github.com/bwasty/teth/blob/master/README.md) for more info.

use std::collections::HashMap;

use structopt::StructOpt;

mod lib;
pub use lib::*;

fn main() {
    let _world = WorldState {
        accounts: HashMap::new(),
    };
    // dbg!(world);

    let _acc = AccountState::default();
    // dbg!(acc);

    let opt = Options::from_args();
    match opt.commands {
        SubCommand::Node { bootstrap } => {
            if bootstrap {
                println!("not implemented yet, but here's the genesis block: {:?}",
                    Block::genesis_block())
            }
            else {
                // TODO!: connect to master node (teth.malkut.net / localhost -> arg...)
                unimplemented!()
            }
        },
        SubCommand::Account { account } => {
            match account {
                AccountSubCommand::Show { address } => {
                    dbg!(address);
                    unimplemented!() // TODO!
                },
                AccountSubCommand::List => {
                    unimplemented!() // TODO!
                }
            }
        },
        SubCommand::Transaction {} => {
            unimplemented!() // TODO!
        },
        SubCommand::Block {} => {
            unimplemented!() // TODO!
        }
    }
}
