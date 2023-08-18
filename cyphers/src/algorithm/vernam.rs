use super::AlgorithmStrategy;
use anyhow::Result;

pub struct Vernam {
    pub key: u8,
}

impl AlgorithmStrategy for Vernam {
    fn encrypt(&self, _message: &str) -> Result<String> {
        todo!()
    }

    fn decrypt(&self, _cypher: &str) -> Result<String> {
        todo!()
    }
}
