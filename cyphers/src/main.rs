pub mod algorithm;
pub mod cli;
pub mod constants;
pub mod cryptanalysis;

use anyhow::Result;
use std::{fs, io::Read};

use algorithm::{AlgorithmStrategy, Cesar, Vernam};
use cryptanalysis::Cryptanalysis;

fn main() -> Result<()> {
    let args = cli::parse();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let output = match args.algorithm {
        cli::Commands::Cesar { key, mode } => {
            let algorithm = Cesar { key };
            execute_algorithm(algorithm, mode, &input)
        }
        cli::Commands::Vernam { key_file, mode } => {
            let key = fs::read(key_file)?;
            let algorithm = Vernam { key };
            execute_algorithm(algorithm, mode, &input)
        }
        cli::Commands::Cryptanalysis => {
            let analyser = Cryptanalysis::new();
            analyser.analyse(input.trim())
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
