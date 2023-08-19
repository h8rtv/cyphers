use super::AlgorithmStrategy;
use anyhow::Result;
use deunicode::deunicode;

pub struct Cesar {
    pub key: u8,
}

impl AlgorithmStrategy for Cesar {
    fn encrypt(&self, message: &str) -> Result<String> {
        let mut cypher = String::new();
        let message = deunicode(message);
        for c in message.chars() {
            if c.is_ascii_alphanumeric() {
                let before = c as u8;
                let shifted = (before - b'0' + self.key) % 75;
                let mut shifted_only_alphanum = shifted;
                if shifted > (b'9' - b'0') && before <= b'9' {
                    shifted_only_alphanum += b'A' - b'9' - 1;
                }
                if shifted > (b'Z' - b'0') && before <= b'Z' {
                    shifted_only_alphanum += b'a' - b'Z' - 1;
                }
                let encrypted_char = (b'0' + shifted_only_alphanum) as char;
                cypher.push(encrypted_char);
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
            if c.is_ascii_alphanumeric() {
                let before = c as u8;
                let shifted = (before - b'0' + 75 - self.key) % 75 + 75;
                let mut shifted_only_alphanum = shifted;
                if shifted < 75 + b'a' - b'0' && before >= b'a' {
                    shifted_only_alphanum -= b'a' - b'Z'- 1;
                }
                if shifted < 75 + b'A' - b'0' && before >= b'A' {
                    shifted_only_alphanum -= b'A' - b'9' - 1;
                }
                let decrypted_char = (b'0' + shifted_only_alphanum % 75) as char;
                message.push(decrypted_char);
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
