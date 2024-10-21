use hex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plaintext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = "ICE";

    let encrypted = repeating_key_xor(plaintext.as_bytes(), key.as_bytes());
    let hex_result = hex::encode(&encrypted);

    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    println!("Encrypted result:");
    println!("{}", hex_result);
    println!("\nVerification:");
    println!("Match expected: {}", hex_result == expected);

    Ok(())
}

fn repeating_key_xor(text: &[u8], key: &[u8]) -> Vec<u8> {
    text.iter()
        .zip(key.iter().cycle())
        .map(|(&t, &k)| t ^ k)
        .collect()
}
