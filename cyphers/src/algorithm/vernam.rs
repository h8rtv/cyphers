use anyhow::{anyhow, Result};
use deunicode::deunicode;
use std::iter::zip;

use super::AlgorithmStrategy;

pub struct Vernam {
    pub key: Vec<u8>,
}

impl AlgorithmStrategy for Vernam {
    fn encrypt(&self, message: &str) -> Result<String> {
        let message = deunicode(message);
        let mut cyphered = String::new();
        let message_bytes = message.bytes();
        if message_bytes.len() < self.key.len() {
            let err_message = format!(
                "The message is smaller than the key. message: {} != key: {}",
                message_bytes.len(),
                self.key.len(),
            );
            return Err(anyhow!(err_message));
        }
        for (c, k) in zip(message_bytes, &self.key) {
            let shifted_k = k - b'A';
            let shifted_c = c - b'A';
            let cyphered_byte = (shifted_c + shifted_k) % 26 + b'A';
            cyphered.push(cyphered_byte as char);
        }
        Ok(cyphered)
    }

    fn decrypt(&self, _cypher: &str) -> Result<String> {
        todo!()
    }
}
