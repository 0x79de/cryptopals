use std::collections::HashMap;

fn main() {
    let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let hex_bytes = hex::decode(hex_string).unwrap();

    let mut best_score = 0;
    let mut best_result = Vec::new();
    let mut best_key = 0;

    for key in 0..=255 {
        let result = xor_single_byte(&hex_bytes, key);
        let score = score_plaintext(&result);

        if score > best_score {
            best_score = score;
            best_result = result;
            best_key = key;
        }
    }

    let decrypted_message = String::from_utf8_lossy(&best_result);
    println!("Decrypted message: {}", decrypted_message);
    println!("Key: {}", best_key as char);
}

fn xor_single_byte(input: &[u8], key: u8) -> Vec<u8> {
    input.iter().map(|&byte| byte ^ key).collect()
}

fn score_plaintext(text: &[u8]) -> usize {
    let mut score = 0;

    let mut char_frequencies = HashMap::new();
    char_frequencies.insert('e', 13);
    char_frequencies.insert('t', 12);
    char_frequencies.insert('a', 11);
    char_frequencies.insert('o', 10);
    char_frequencies.insert('i', 9);
    char_frequencies.insert('n', 8);
    char_frequencies.insert(' ', 7);
    char_frequencies.insert('s', 6);
    char_frequencies.insert('h', 5);
    char_frequencies.insert('r', 4);
    char_frequencies.insert('d', 3);
    char_frequencies.insert('l', 2);
    char_frequencies.insert('u', 1);

    for &byte in text {
        if let Some(c) = std::char::from_u32(byte as u32) {
            let lowercase_c = c.to_lowercase().next().unwrap();
            if char_frequencies.contains_key(&lowercase_c) {
                score += char_frequencies[&lowercase_c];
            }
        }
    }

    score
}
