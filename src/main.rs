use std::collections::HashMap;

use structopt::StructOpt;

use ethereum_types::{Address};

mod lib;
use lib::*;

#[derive(Debug, StructOpt)]
#[structopt(
    about = "A Toy Ethereum implementation.", 
    author = "",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp", 
        setting = "structopt::clap::AppSettings::DeriveDisplayOrder")
)]
struct Opt {
    #[structopt(subcommand)]  
    commands: SubCommand
}

#[derive(StructOpt, Debug)]
enum SubCommand {
    /// Run a node.
    #[structopt(name = "node", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Node {
        /// Bootstrap the chain (with the genesis block).
        #[structopt(long)]
        bootstrap: bool,
    },
    /// Show information about accounts.
    #[structopt(name = "account", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Account {
        #[structopt(subcommand)]  
        account: AccountSubCommand
    },
    /// Show information about transactions.
    #[structopt(name = "transaction", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Transaction {
        // TODO!!
    },
    /// Show information about blocks.
    #[structopt(name = "block", raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
    Block {
        // TODO!!
    }
}

#[derive(StructOpt, Debug)]
enum AccountSubCommand {
    /// Show details of account (balance etc.)
    #[structopt(name = "show")]
    Show {
        address: Address
    },
    /// List accounts, ordered by balance (descending)
    #[structopt(name = "list")]
    List 
}

fn main() {
    let world = WorldState { accounts: HashMap::new() };
    dbg!(world);

    let acc = AccountState::default();
    dbg!(acc);

    let opt = Opt::from_args();
    println!("{:?}", opt);
}
