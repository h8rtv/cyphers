use anyhow::Result;
use deunicode::deunicode;

use super::AlgorithmStrategy;
use crate::constants::{ALPHANUM_CHARS, ALPHANUM_CHARS_LEN};

pub struct Cesar {
    pub key: u8,
}

impl AlgorithmStrategy for Cesar {
    fn encrypt(&self, message: &str) -> Result<String> {
        let mut cypher = String::new();
        let message = deunicode(message);
        for c in message.chars() {
            if let Some(base_char_index) = ALPHANUM_CHARS.find(c) {
                let cyphered_char_index =
                    (base_char_index as i32 + self.key as i32) % ALPHANUM_CHARS_LEN as i32;
                let cyphered_char = ALPHANUM_CHARS
                    .chars()
                    .nth(cyphered_char_index as usize)
                    .expect("the cyphered ascii char should be in the array");
                cypher.push(cyphered_char);
            } else {
                cypher.push(c);
            }
        }
        Ok(cypher)
    }

    fn decrypt(&self, cypher: &str) -> Result<String> {
        let mut message = String::new();
        let cypher = deunicode(cypher);
        for c in cypher.chars() {
            if let Some(base_char_index) = ALPHANUM_CHARS.find(c) {
                let message_char_index = (base_char_index as i32 + ALPHANUM_CHARS_LEN as i32
                    - (self.key as i32 % ALPHANUM_CHARS_LEN as i32) as i32)
                    % ALPHANUM_CHARS_LEN as i32;
                let cyphered_char = ALPHANUM_CHARS
                    .chars()
                    .nth(message_char_index as usize)
                    .expect("the message ascii char should be in the array");
                message.push(cyphered_char);
            } else {
                message.push(c);
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
            let cesar = Cesar { key: 1 };
            let message = String::from("A");
            let expected_cypher = String::from("B");
            let cypher = cesar
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }
        #[test]
        fn encrypt_upper() {
            let cesar = Cesar { key: 10 };
            let message = String::from("TESTE");
            let expected_cypher = String::from("dOcdO");
            let cypher = cesar
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }
        #[test]
        fn encrypt_lower() {
            let cesar = Cesar { key: 10 };
            let message = String::from("teste");
            let expected_cypher = String::from("3o23o");
            let cypher = cesar
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }
        #[test]
        fn encrypt_num() {
            let cesar = Cesar { key: 10 };
            let message = String::from("12345");
            let expected_cypher = String::from("BCDEF");
            let cypher = cesar
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }

        #[test]
        fn encrypt_diacritics() {
            let cesar = Cesar { key: 10 };
            let message = String::from("TÉÑçá");
            let expected_cypher = String::from("dOXmk");
            let cypher = cesar
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }

        #[test]
        fn encrypt_whitespaces() {
            let cesar = Cesar { key: 10 };
            let message = String::from("12345 TÉÑçá teste");
            let expected_cypher = String::from("BCDEF dOXmk 3o23o");
            let cypher = cesar
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }

        #[test]
        fn encrypt_highend() {
            let cesar = Cesar { key: 10 };
            let message = String::from("vwxyz");
            let expected_cypher = String::from("56789");
            let cypher = cesar
                .encrypt(&message)
                .expect("Test message should be alphanumeric");
            assert_eq!(cypher, expected_cypher);
        }
    }

    mod decrypt {
        use super::*;
        #[test]
        fn decrypt_simple() {
            let cesar = Cesar { key: 1 };
            let cypher = String::from("B");
            let expected_message = String::from("A");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }
        #[test]
        fn decrypt_upper() {
            let cesar = Cesar { key: 10 };
            let cypher = String::from("dOcdO");
            let expected_message = String::from("TESTE");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }
        #[test]
        fn decrypt_lower() {
            let cesar = Cesar { key: 10 };
            let cypher = String::from("3o23o");
            let expected_message = String::from("teste");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }
        #[test]
        fn decrypt_num() {
            let cesar = Cesar { key: 10 };
            let cypher = String::from("BCDEF");
            let expected_message = String::from("12345");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }

        #[test]
        fn decrypt_diacritics() {
            let cesar = Cesar { key: 10 };
            let cypher = String::from("dOXmk");
            let expected_message = String::from("TENca");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }

        #[test]
        fn decrypt_whitespaces() {
            let cesar = Cesar { key: 10 };
            let cypher = String::from("BCDEF dOXmk 3o23o");
            let expected_message = String::from("12345 TENca teste");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }

        #[test]
        fn decrypt_highend() {
            let cesar = Cesar { key: 10 };
            let cypher = String::from("56789");
            let expected_message = String::from("vwxyz");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }
    }
}
