pub mod algorithm;
pub mod cli;
pub mod cryptanalysis;
pub mod utils;

use anyhow::{Result, anyhow};
use std::{fs, io::Read};

use algorithm::{AlgorithmStrategy, Cesar, Vernam};
use cryptanalysis::Cryptanalysis;
use utils::generate_vernam_key;

fn main() -> Result<()> {
    let args = cli::parse();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let output = match args.algorithm {
        cli::Commands::Cesar { key, mode } => {
            let algorithm = Cesar { key };
            execute_algorithm(algorithm, mode, &input)
        }
        cli::Commands::Vernam { mode, key_group } => {
            let key = match key_group.key_file {
                Some(key_file) => Ok(fs::read(key_file)?),
                None if mode.cypher => Ok(generate_vernam_key(input.trim().len())),
                None => Err(anyhow!("Can't generate key on decypher mode")),
            }?;
            if let Some(out_key) = key_group.out_key_file {
                fs::write(out_key, &key)?;
            }
            let algorithm = Vernam { key };
            execute_algorithm(algorithm, mode, &input)
        }
        cli::Commands::Cryptanalysis => {
            let analyser = Cryptanalysis::new();
            analyser.analyse(input.trim());
            Ok(String::new())
        }
    }?;

    println!("{}", output);
    Ok(())
}

fn execute_algorithm<T: AlgorithmStrategy>(
    algorithm: T,
    mode: cli::GroupMode,
    input: &String,
) -> Result<String> {
    match mode.cypher {
        true => algorithm.encrypt(input.trim()),
        false => algorithm.decrypt(input.trim()),
    }
}
