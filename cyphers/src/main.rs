pub mod algorithm;
pub mod cli;

use std::io::Read;
use anyhow::Result;

use algorithm::{Algorithm, AlgorithmStrategy, Cesar, Vernam};

fn main() -> Result<()> {
    let args = cli::parse();
    let key = args.key;

    let algorithm = match &*args.algorithm {
        "cesar" => Algorithm::Cesar(Cesar { key }),
        "vernam" => Algorithm::Vernam(Vernam { key }),
        _ => panic!("Algorithm not found"),
    };

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let output = match args.mode.cypher {
        true => algorithm.encrypt(input.trim()),
        false => algorithm.decrypt(input.trim()),
    }?;

    println!("{}", output);
    Ok(())
}
