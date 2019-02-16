//! Toy Ethereum client closely following the [Yellow Paper](https://github.com/ethereum/yellowpaper/) ([PDF](https://ethereum.github.io/yellowpaper/paper.pdf)).
//! Structs, fields and methods are annotated with their formal definition where applicable.
//! See [README.md](https://github.com/bwasty/teth/blob/master/README.md) for more info.

use std::collections::HashMap;

use structopt::StructOpt;
use ethereum_types::Address;

mod lib;
pub use lib::*;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "A Toy Ethereum implementation.",
    author = "",
    raw(
        setting = "structopt::clap::AppSettings::ColoredHelp",
        setting = "structopt::clap::AppSettings::DeriveDisplayOrder"
    )
)]
struct Opt {
    #[structopt(subcommand)]
    commands: SubCommand,
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Run a node.
    #[structopt(
        name = "node",
        author = "",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp"),
        raw(setting = "structopt::clap::AppSettings::DisableVersion")
    )]
    Node {
        /// Bootstrap the chain (with the genesis block).
        #[structopt(long)]
        bootstrap: bool,
    },

    /// Query information about accounts.
    #[structopt(
        name = "account",
        author = "",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp"),
        raw(setting = "structopt::clap::AppSettings::DisableVersion")
    )]
    Account {
        #[structopt(subcommand)]
        account: AccountSubCommand,
    },

    /// Query information about transactions.
    #[structopt(
        name = "transaction",
        author = "",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp"),
        raw(setting = "structopt::clap::AppSettings::DisableVersion")
    )]
    Transaction {
        // TODO!
    },

    /// Query information about blocks.
    #[structopt(
        name = "block",
        author = "",
        raw(setting = "structopt::clap::AppSettings::ColoredHelp"),
        raw(setting = "structopt::clap::AppSettings::DisableVersion")
    )]
    Block {
        // TODO!
    },
}

#[derive(StructOpt, Debug)]
enum AccountSubCommand {
    /// Show details of account (balance etc.)
    #[structopt(
        name = "show",
        raw(setting = "structopt::clap::AppSettings::DisableVersion")
    )]
    Show { address: Address },
    /// List accounts, ordered by balance (descending)
    #[structopt(
        name = "list",
        raw(setting = "structopt::clap::AppSettings::DisableVersion")
    )]
    List,
}

fn main() {
    let _world = WorldState {
        accounts: HashMap::new(),
    };
    // dbg!(world);

    let _acc = AccountState::default();
    // dbg!(acc);

    let opt = Opt::from_args();
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
