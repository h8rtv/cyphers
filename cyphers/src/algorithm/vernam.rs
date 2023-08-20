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
        if message.len() < self.key.len() {
            let err_message = format!(
                "The message is smaller than the key. message: {} != key: {}",
                message.len(),
                self.key.len(),
            );
            return Err(anyhow!(err_message));
        }
        for (c, k) in zip(message.bytes(), &self.key) {
            let shifted_k = k - b'A';
            let shifted_c = c - b'A';
            let cyphered_byte = (shifted_c + shifted_k) % 26 + b'A';
            cyphered.push(cyphered_byte as char);
        }
        Ok(cyphered)
    }

    fn decrypt(&self, cypher: &str) -> Result<String> {
        let cypher = deunicode(cypher);
        let mut message = String::new();
        if cypher.len() < self.key.len() {
            let err_message = format!(
                "The cypher is smaller than the key. message: {} != key: {}",
                cypher.len(),
                self.key.len(),
            );
            return Err(anyhow!(err_message));
        }
        for (c, k) in zip(cypher.bytes(), &self.key) {
            let shifted_k = k - b'A';
            let shifted_c = c - b'A';
            let message_byte = (shifted_c + 26 - shifted_k) % 26 + b'A';
            message.push(message_byte as char);
        }
        Ok(message)
    }
}
