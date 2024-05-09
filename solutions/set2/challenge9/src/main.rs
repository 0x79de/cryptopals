fn pkcs7_padding(input: &[u8], block_size: usize) -> Vec<u8> {
    let padding_length = block_size - (input.len() % block_size);
    let mut padded_input = input.to_vec();

    for _ in 0..padding_length {
        padded_input.push(padding_length as u8);
    }

    padded_input
}

fn main() {
    let plaintext = b"YELLOW SUBMARINE";
    let padded_plaintext = pkcs7_padding(plaintext, 20);

    println!("Padded plaintext: {:?}", padded_plaintext);
}
