use anyhow::{anyhow, Result};
use deunicode::deunicode;
use std::iter::zip;

use crate::utils::{roll, ALPHANUM_CHARS};

use super::AlgorithmStrategy;

pub struct Vernam {
    pub key: Vec<u8>,
}

impl AlgorithmStrategy for Vernam {
    fn encrypt(&self, message: &str) -> Result<String> {
        let message = deunicode(message);
        let mut cyphered = String::new();
        if message.len() > self.key.len() {
            let err_message = format!(
                "The message is bigger than the key. message: {} > key: {}",
                message.len(),
                self.key.len(),
            );
            return Err(anyhow!(err_message));
        }
        for (c, k) in zip(message.chars(), &self.key) {
            if let Some(key) = ALPHANUM_CHARS.find(*k as char) {
                match roll(c, key as i32) {
                    Some(encoded_char) => cyphered.push(encoded_char),
                    None => cyphered.push(c),
                };
            } else {
                return Err(anyhow!("Unsupported key"));
            }
        }
        Ok(cyphered)
    }

    fn decrypt(&self, cypher: &str) -> Result<String> {
        let cypher = deunicode(cypher);
        let mut message = String::new();
        if cypher.len() > self.key.len() {
            let err_message = format!(
                "The cypher is bigger than the key. cypher: {} > key: {}",
                cypher.len(),
                self.key.len(),
            );
            return Err(anyhow!(err_message));
        }
        for (c, k) in zip(cypher.chars(), &self.key) {
            if let Some(key) = ALPHANUM_CHARS.find(*k as char) {
                match roll(c, key as i32 * -1) {
                    Some(decoded_char) => message.push(decoded_char),
                    None => message.push(c),
                };
            } else {
                return Err(anyhow!("Unsupported key"));
            }
        }
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encrypt {
        use super::*;
        #[test]
        fn encrypt_simple() {
            let vernam = Vernam { key: vec![b'1', b'2', b'3', b'4', b'5'] };
            let message = String::from("ABCDE");
            let expected_cypher = String::from("BDFHJ");
            let cypher = vernam
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }
        #[test]
        fn encrypt() {
            let vernam = Vernam {
                key: vec![b'a', b'b', b'c', b'6', b'9'],
            };
            let message = String::from("HELLO");
            let expected_cypher = String::from("rpxRX");
            let cypher = vernam
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }
    }

    mod decrypt {
        use super::*;
        #[test]
        fn decrypt_simple() {
            let vernam = Vernam { key: vec![b'1', b'2', b'3', b'4', b'5'] };
            let cypher = String::from("BDFHJ");
            let expected_message = String::from("ABCDE");
            let message = vernam
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }
        #[test]
        fn decrypt() {
            let vernam = Vernam {
                key: vec![b'a', b'b', b'c', b'6', b'9'],
            };
            let cypher = String::from("rpxRX");
            let expected_message = String::from("HELLO");
            let message = vernam
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }
    }
}
