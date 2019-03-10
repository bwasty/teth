use structopt::StructOpt;

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
}
