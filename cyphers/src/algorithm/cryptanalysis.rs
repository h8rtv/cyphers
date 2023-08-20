use anyhow::Result;
use deunicode::deunicode;
use std::collections::HashMap;

use super::AlgorithmStrategy;
use super::Cesar;

pub struct Cryptanalysis {
    pub dict: HashMap<char, f32>,
}

fn portuguese_dict() -> HashMap<char, f32> {
    let mut letter_freq_map: HashMap<char, f32> = HashMap::new();

    letter_freq_map.insert('A', 14.63);
    letter_freq_map.insert('B', 1.04);
    letter_freq_map.insert('C', 3.88);
    letter_freq_map.insert('D', 4.99);
    letter_freq_map.insert('E', 12.57);
    letter_freq_map.insert('F', 1.02);
    letter_freq_map.insert('G', 1.30);
    letter_freq_map.insert('H', 1.28);
    letter_freq_map.insert('I', 6.18);
    letter_freq_map.insert('J', 0.40);
    letter_freq_map.insert('K', 0.02);
    letter_freq_map.insert('L', 2.78);
    letter_freq_map.insert('M', 4.74);
    letter_freq_map.insert('N', 5.05);
    letter_freq_map.insert('O', 10.73);
    letter_freq_map.insert('P', 2.52);
    letter_freq_map.insert('Q', 1.20);
    letter_freq_map.insert('R', 6.53);
    letter_freq_map.insert('S', 7.81);
    letter_freq_map.insert('T', 4.34);
    letter_freq_map.insert('U', 4.63);
    letter_freq_map.insert('V', 1.67);
    letter_freq_map.insert('W', 0.01);
    letter_freq_map.insert('X', 0.21);
    letter_freq_map.insert('Y', 0.01);
    letter_freq_map.insert('Z', 0.47);

    letter_freq_map
}

impl Cryptanalysis {
    pub fn new() -> Self {
        Cryptanalysis {
            dict: portuguese_dict(),
        }
    }
}

pub const ALPHANUM_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub const ALPHANUM_CHARS_LEN: usize = 62;

impl AlgorithmStrategy for Cryptanalysis {
    fn encrypt(&self, _message: &str) -> Result<String> {
        // TODO: Repensar código pq isso é palhaçada
        unreachable!()
    }

    fn decrypt(&self, cypher: &str) -> Result<String> {
        let mut char_buckets = Vec::with_capacity(ALPHANUM_CHARS_LEN);
        char_buckets.resize(ALPHANUM_CHARS_LEN, 0);

        let cypher = deunicode(cypher);
        let mut total_length = 0;
        for c in cypher.chars() {
            if let Some(char_pos) = ALPHANUM_CHARS.find(c) {
                char_buckets[char_pos] += 1;
                total_length += 1;
            }
        }
        let mut sorted_freqs: Vec<(usize, f32)> = char_buckets
            .iter()
            .enumerate()
            .map(|(index, value)| {
                (
                    index,
                    *value as f32 / total_length as f32,
                )
            })
            .collect();

        sorted_freqs.sort_by(|a, b| b.1.total_cmp(&a.1));

        for (index, freq) in &sorted_freqs {
            let c = ALPHANUM_CHARS.chars().nth(*index).unwrap();
            println!("{:2} {:} {:5.2}%", index, c, freq * 100.0);
        }
        let predicted_key = 17;

        let predicted_out = Cesar { key: predicted_key }.decrypt(&cypher)?;
        Ok(predicted_out)
    }
}
