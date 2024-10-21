use std::collections::HashMap;

fn score_english(text: &[u8]) -> f64 {
    let freq: HashMap<char, f64> = [
        ('a', 8.2), ('b', 1.5), ('c', 2.8), ('d', 4.3),
        ('e', 13.0), ('f', 2.2), ('g', 2.0), ('h', 6.1),
        ('i', 7.0), ('j', 0.15), ('k', 0.77), ('l', 4.0),
        ('m', 2.4), ('n', 6.7), ('o', 7.5), ('p', 1.9),
        ('q', 0.095), ('r', 6.0), ('s', 6.3), ('t', 9.1),
        ('u', 2.8), ('v', 0.98), ('w', 2.4), ('x', 0.15),
        ('y', 2.0), ('z', 0.074), (' ', 13.0)
    ].iter().cloned().collect();

    let text_lower: String = text.iter()
        .filter(|&&b| b.is_ascii())
        .map(|&b| b as char)
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let total_chars = text_lower.len() as f64;
    let mut score = 0.0;

    for c in text_lower.chars() {
        if let Some(&expected_freq) = freq.get(&c) {
            score += expected_freq;
        }
    }

    score / total_chars
}

fn single_byte_xor(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&b| b ^ key).collect()
}

fn detect_single_char_xor(ciphertext: &[u8]) -> (u8, Vec<u8>, f64) {
    let mut best_score = f64::NEG_INFINITY;
    let mut best_key = 0;
    let mut best_plaintext = Vec::new();

    for key in 0..=255 {
        let plaintext = single_byte_xor(ciphertext, key);
        let score = score_english(&plaintext);

        if score > best_score {
            best_score = score;
            best_key = key;
            best_plaintext = plaintext;
        }
    }

    (best_key, best_plaintext, best_score)
}

fn main() {
    let lines: Vec<String> = include_str!("input.txt")
        .lines()
        .map(|s| s.trim().to_string())
        .collect();

    let mut best_line = 0;
    let mut best_score = f64::NEG_INFINITY;
    let mut best_result = (0, Vec::new(), 0.0);

    for (i, line) in lines.iter().enumerate() {
        if let Ok(bytes) = base64::decode(line) {
            let result = detect_single_char_xor(&bytes);
            
            if result.2 > best_score {
                best_score = result.2;
                best_line = i;
                best_result = result;
            }
        }
    }

    println!("Encrypted line number: {}", best_line + 1);
    println!("Key used: {}", best_result.0);
    println!("Decrypted text: {}", String::from_utf8_lossy(&best_result.1));
}