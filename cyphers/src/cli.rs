use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(flatten)]
    pub mode: GroupMode,

    /// Inform key
    #[arg(short, long)]
    pub key: u8,

    /// Inform algorithm
    #[arg(short, long)]
    pub algorithm: String,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct GroupMode {
    /// Select cyphre mode
    #[arg(short, long)]
    pub cypher: bool,

    /// Select decypher mode
    #[arg(short, long)]
    pub decypher: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
