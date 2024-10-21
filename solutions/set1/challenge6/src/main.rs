use std::collections::HashMap;
use base64;

fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter()
        .zip(s2.iter())
        .map(|(&x, &y)| (x ^ y).count_ones())
        .sum()
}

fn score_english(text: &[u8]) -> f64 {
    let freq: HashMap<char, f64> = [
        ('a', 8.2),
        ('b', 1.5),
        ('c', 2.8),
        ('d', 4.3),
        ('e', 13.0),
        ('f', 2.2),
        ('g', 2.0),
        ('h', 6.1),
        ('i', 7.0),
        ('j', 0.15),
        ('k', 0.77),
        ('l', 4.0),
        ('m', 2.4),
        ('n', 6.7),
        ('o', 7.5),
        ('p', 1.9),
        ('q', 0.095),
        ('r', 6.0),
        ('s', 6.3),
        ('t', 9.1),
        ('u', 2.8),
        ('v', 0.98),
        ('w', 2.4),
        ('x', 0.15),
        ('y', 2.0),
        ('z', 0.074),
        (' ', 13.0),
    ]
    .iter()
    .cloned()
    .collect();

    let text_lower: String = text
        .iter()
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

fn find_keysize(data: &[u8]) -> Vec<(usize, f64)> {
    let mut scores = Vec::new();

    for keysize in 2..=40 {
        if data.len() < keysize * 4 {
            continue;
        }

        let mut distances = Vec::new();
        let chunks: Vec<_> = data.chunks(keysize).take(4).collect();

        for i in 0..3 {
            for j in (i + 1)..4 {
                let dist = hamming_distance(chunks[i], chunks[j]) as f64;
                distances.push(dist / keysize as f64);
            }
        }

        let avg_distance = distances.iter().sum::<f64>() / distances.len() as f64;
        scores.push((keysize, avg_distance));
    }

    scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    scores
}

fn single_byte_xor(data: &[u8], key: u8) -> Vec<u8> {
    data.iter().map(|&b| b ^ key).collect()
}

fn find_single_byte_key(data: &[u8]) -> (u8, f64) {
    let mut best_score = f64::NEG_INFINITY;
    let mut best_key = 0;

    for key in 0..=255 {
        let decrypted = single_byte_xor(data, key);
        let score = score_english(&decrypted);

        if score > best_score {
            best_score = score;
            best_key = key;
        }
    }

    (best_key, best_score)
}

fn break_repeating_xor(data: &[u8]) -> Vec<u8> {
    let keysizes = find_keysize(data);
    let keysize = keysizes[0].0; // Take the most likely keysize

    let mut key = vec![0; keysize];

    for i in 0..keysize {
        let block: Vec<u8> = data.iter().skip(i).step_by(keysize).cloned().collect();

        key[i] = find_single_byte_key(&block).0;
    }

    key
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test Hamming distance
    let s1 = b"this is a test";
    let s2 = b"wokka wokka!!!";
    let distance = hamming_distance(s1, s2);
    println!("Hamming distance test: {}", distance);
    assert_eq!(distance, 37);

    let content = std::fs::read_to_string("6.txt")?;
    let encrypted_data = base64::decode(content.trim())?;

    let key = break_repeating_xor(&encrypted_data);
    println!("\nFound key: {}", String::from_utf8_lossy(&key));

    let decrypted = encrypted_data
        .iter()
        .zip(key.iter().cycle())
        .map(|(&d, &k)| d ^ k)
        .collect::<Vec<u8>>();

    println!(
        "\nDecrypted message:\n{}",
        String::from_utf8_lossy(&decrypted)
    );

    Ok(())
}
