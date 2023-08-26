use rand::Rng;

pub const ALPHANUM_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const ALPHANUM_CHARS_LEN: usize = 62;

pub fn roll(c: char, amount: i32) -> Option<char> {
    if let Some(index) = ALPHANUM_CHARS.find(c) {
        let key = amount % ALPHANUM_CHARS_LEN as i32;
        let out_index =
            (index as i32 + ALPHANUM_CHARS_LEN as i32 + key as i32) % ALPHANUM_CHARS_LEN as i32;
        let out = ALPHANUM_CHARS
            .chars()
            .nth(out_index as usize)
            .expect("The ascii char should be in the array");
        return Some(out);
    }
    return None;
}

pub fn generate_vernam_key(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key = (0..len)
        .map(|_| rng.gen_range(0..ALPHANUM_CHARS_LEN as u8))
        .map(|idx| {
            ALPHANUM_CHARS
                .chars()
                .nth(idx as usize)
                .expect("The ascii char should be in the array") as u8
        })
        .collect();
    key
}
