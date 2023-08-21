use std::iter::zip;

use anyhow::Result;
use deunicode::deunicode;

use crate::algorithm::{AlgorithmStrategy, Cesar};
use crate::constants::{ALPHANUM_CHARS, ALPHANUM_CHARS_LEN};

pub struct Cryptanalysis {
    dict: Vec<(char, f32)>,
}

fn portuguese_dict() -> Vec<(char, f32)> {
    vec![
        ('a', 0.1463),
        ('e', 0.1257),
        ('o', 0.1073),
        ('r', 0.0653),
        ('i', 0.0618),
        ('s', 0.0781),
        ('n', 0.0505),
        ('d', 0.0499),
        ('t', 0.0434),
        ('u', 0.0463),
        ('m', 0.0474),
        ('c', 0.0388),
        ('l', 0.0278),
        ('p', 0.0252),
        ('g', 0.0130),
        ('h', 0.0128),
        ('q', 0.0120),
        ('v', 0.0167),
        ('f', 0.0102),
        ('b', 0.0104),
        ('j', 0.0040),
        ('z', 0.0047),
        ('x', 0.0021),
        ('k', 0.0002),
        ('w', 0.0001),
        ('y', 0.0001),
    ]
}

fn print_table_of_freqs(freqs: &Vec<(usize, f32)>) {
    println!("{: <5} | {: <5} | {: <5}", "index", "char", "frequency");
    for (index, freq) in freqs.iter().filter(|x| x.1 > 0.0) {
        let c = ALPHANUM_CHARS.chars().nth(*index).unwrap();
        println!("{:<5} | {:<5} | {:5.2}%", index, c, freq * 100.0);
    }
}

impl Cryptanalysis {
    pub fn new() -> Self {
        Cryptanalysis {
            dict: portuguese_dict(),
        }
    }
}

impl Cryptanalysis {
    pub fn analyse(&self, cypher: &str) -> Result<String> {
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
            .map(|(index, value)| (index, *value as f32 / total_length as f32))
            .collect();

        sorted_freqs.sort_by(|a, b| b.1.total_cmp(&a.1));

        print_table_of_freqs(&sorted_freqs);

        for (sample_data, dict_data) in zip(&sorted_freqs, &self.dict) {
            let (sample_index, _) = sample_data;
            let (dict_char, _) = dict_data;

            let c = ALPHANUM_CHARS.chars().nth(*sample_index).unwrap();
            let c = c.to_ascii_lowercase();

            let diff = (c as i32 - *dict_char as i32).abs();
            println!(
                "Char dict: {}, Char cypher: {}, Diff: {}",
                dict_char, c, diff
            );
        }

        let predicted_key = 17;

        let predicted_out = Cesar { key: predicted_key }.decrypt(&cypher)?;
        Ok(predicted_out)
    }
}
