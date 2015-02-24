use utilities::*;
fn repeating_key_xor_encode(start: &str, key: &str) -> Vec<u8> {
    bytes2hex(
        start
        .bytes()
        .zip(key.bytes().cycle())
        .map(
            |(x, y)| x ^ y
        )
        .collect()
    )
}

pub fn repeating_key_xor_encode_pretty(start: &str, key: &str) -> String {
    hex2hex_pretty(repeating_key_xor_encode(start, key))
}
