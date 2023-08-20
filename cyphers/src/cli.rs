use clap::{Parser, Subcommand};

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
        /// Inform key file path
        #[arg(short, long)]
        key_file: String,

        #[clap(flatten)]
        mode: GroupMode,
    },
    Cryptanalysis,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct GroupMode {
    /// Select cypher mode
    #[arg(short, long)]
    pub cypher: bool,

    /// Select decypher mode
    #[arg(short, long)]
    pub decypher: bool,
}

pub fn parse() -> Cli {
    Cli::parse()
}
