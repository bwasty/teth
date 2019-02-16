use structopt::StructOpt;

use ethereum_types::Address;

/// CLI options (via structopt)
#[derive(Debug, StructOpt)]
#[structopt(
    about = "A Toy Ethereum implementation.",
    author = "",
    raw(
        setting = "structopt::clap::AppSettings::ColoredHelp",
        setting = "structopt::clap::AppSettings::DeriveDisplayOrder"
    )
)]
pub struct Options {
    #[structopt(subcommand)]
    pub commands: SubCommand,
}

/// CLI subcommands
#[derive(StructOpt, Debug)]
pub enum SubCommand {
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

/// CLI sub-subcommand
#[derive(StructOpt, Debug)]
pub enum AccountSubCommand {
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
