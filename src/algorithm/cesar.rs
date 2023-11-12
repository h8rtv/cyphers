use anyhow::Result;
use deunicode::deunicode;

use super::AlgorithmStrategy;
use crate::utils::roll;

pub struct Cesar {
    pub key: u8,
}

impl AlgorithmStrategy for Cesar {
    fn encrypt(&self, message: &str) -> Result<String> {
        let mut cypher = String::with_capacity(message.len());
        let message = deunicode(message);
        for c in message.chars() {
            match roll(c, self.key as i32) {
                Some(encoded_char) => cypher.push(encoded_char),
                None => cypher.push(c),
            };
        }
        Ok(cypher)
    }

    fn decrypt(&self, cypher: &str) -> Result<String> {
        let mut message = String::with_capacity(cypher.len());
        let cypher = deunicode(cypher);
        for c in cypher.chars() {
            match roll(c, self.key as i32 * -1) {
                Some(decoded_char) => message.push(decoded_char),
                None => message.push(c),
            };
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

        #[test]
        fn a() {
            let cesar = Cesar { key: 12 };
            let cypher = String::from("00000");
            let expected_message = String::from("ooooo");
            let message = cesar
                .decrypt(&cypher)
                .expect("Test message should be alphanumeric");
            assert_eq!(message, expected_message);
        }
    }
}
