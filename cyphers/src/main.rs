pub mod algorithm;
pub mod cli;

use anyhow::Result;
use std::{fs, io::Read};

use algorithm::{Algorithm, AlgorithmStrategy, Cesar, Cryptanalysis, Vernam};

fn main() -> Result<()> {
    let args = cli::parse();

    let (algorithm, is_cypher) = match args.algorithm {
        cli::Commands::Cesar { key, mode } => (Algorithm::Cesar(Cesar { key }), mode.cypher),
        cli::Commands::Vernam { key_file, mode } => {
            let key = fs::read(key_file)?;
            (Algorithm::Vernam(Vernam { key }), mode.cypher)
        }
        cli::Commands::Cryptanalysis => {
            (Algorithm::Cryptanalysis(Cryptanalysis::new()), false)
        }
    };

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let output = match is_cypher {
        true => algorithm.encrypt(input.trim()),
        false => algorithm.decrypt(input.trim()),
    }?;

    println!("{}", output);
    Ok(())
}
