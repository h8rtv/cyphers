use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Inform algorithm
    #[command(subcommand)]
    pub algorithm: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Cesar {
        /// Inform key
        #[arg(short, long)]
        key: u8,

        #[clap(flatten)]
        mode: GroupMode,
    },
    Vernam {
        #[clap(flatten)]
        key_group: GroupVernamKey,

        #[clap(flatten)]
        mode: GroupMode,
    },
    Cryptanalysis,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct GroupMode {
    /// Select cypher mode
    #[arg(short, long)]
    pub cypher: bool,

    /// Select decypher mode
    #[arg(short, long)]
    pub decypher: bool,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
pub struct GroupVernamKey {
    /// Inform key file path
    #[arg(short, long)]
    pub key_file: Option<String>,

    /// Inform name of the output of the key that will be generated
    #[arg(short, long)]
    pub out_key_file: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
