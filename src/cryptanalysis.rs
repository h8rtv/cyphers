use crate::utils::{ALPHANUM_CHARS, ALPHANUM_CHARS_LEN};

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

fn print_table_of_freqs_unbounded(freqs: &Vec<(usize, f32)>) {
    println!("{: <5} | {: <5}", "index", "frequency");
    for (index, freq) in freqs.iter().filter(|x| x.1 > 0.0) {
        println!("{:<5} | {:5.2}%", index, freq * 100.0);
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
    pub fn analyse(&self, cypher: &[u8]) {
        let mut char_buckets = vec![0; ALPHANUM_CHARS_LEN];
        char_buckets.resize(ALPHANUM_CHARS_LEN, 0);

        let mut total_length = 0;
        for c in cypher {
            if let Some(char_pos) = ALPHANUM_CHARS.find(*c as char) {
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

        let mut char_buckets_unbounded = vec![0; 256];
        let mut total_length = 0;
        for c in cypher {
            char_buckets_unbounded[*c as usize] += 1;
            total_length += 1;
        }
        let mut sorted_freqs_unbounded: Vec<(usize, f32)> = char_buckets_unbounded
            .iter()
            .enumerate()
            .map(|(index, value)| (index, *value as f32 / total_length as f32))
            .collect();

        sorted_freqs_unbounded.sort_by(|a, b| b.1.total_cmp(&a.1));
        print_table_of_freqs_unbounded(&sorted_freqs_unbounded);

        let mut guesses: Vec<u8> = vec![0; ALPHANUM_CHARS_LEN];
        for (i, sample_data) in sorted_freqs.iter().take(self.dict.len()).enumerate() {
            let (sample_index, _) = sample_data;

            let c = ALPHANUM_CHARS.chars().nth(*sample_index).unwrap();

            let start_index = if i > 2 { i - 3 } else { 0 };

            let end_index = std::cmp::min(i + 3, self.dict.len());

            for (j, dict_data) in self.dict[start_index..end_index].iter().enumerate() {
                let (dict_char, _) = dict_data;

                let dict_index = ALPHANUM_CHARS.find(*dict_char).unwrap();

                let diff = (*sample_index as i32 - dict_index as i32 + ALPHANUM_CHARS_LEN as i32)
                    % ALPHANUM_CHARS_LEN as i32;
                guesses[diff as usize] += 1;
                println!(
                    "i: {}, Char dict: {}, Char cypher: {}, Diff: {}",
                    j, dict_char, c, diff
                );
            }
            println!();
        }
        let mut sorted_guesses: Vec<_> = guesses.iter().enumerate().filter(|a| *a.1 > 0).collect();

        sorted_guesses.sort_by(|a, b| b.1.cmp(&a.1));
        for (i, score) in sorted_guesses {
            println!("Guess: {}, Score: {}", i, score);
        }
    }
}
