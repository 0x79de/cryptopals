use base64::{Engine as _, engine::general_purpose::STANDARD};
use hex;

fn main() {
    let hex_encoded_text = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let decoded_bytes = hex::decode(hex_encoded_text).expect("Failed to decode hex");
    let base64_encoded_text = STANDARD.encode(&decoded_bytes);

    println!("{}", base64_encoded_text);
}
