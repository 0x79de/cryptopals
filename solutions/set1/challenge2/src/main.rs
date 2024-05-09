use hex;

fn fixed_xor(buffer1: &str, buffer2: &str) -> String {
    let bytes1 = hex::decode(buffer1).expect("Invalid hex string");
    let bytes2 = hex::decode(buffer2).expect("Invalid hex string");

    let mut result = Vec::new();

    for (byte1, byte2) in bytes1.iter().zip(bytes2.iter()) {
        result.push(byte1 ^ byte2);
    }

    hex::encode(result)
}

fn main() {
    let buffer1 = "1c0111001f010100061a024b53535009181c";
    let buffer2 = "686974207468652062756c6c277320657965";
    let expected_result = "746865206b696420646f6e277420706c6179";

    let result = fixed_xor(buffer1, buffer2);
    println!("Result: {}", result);
    assert_eq!(result, expected_result);
}
