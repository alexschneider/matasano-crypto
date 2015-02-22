use utilities::*;
fn repeating_key_xor_encode(start: &str, key: &str) -> Vec<u8> {
    let mut key_iter = key.bytes().cycle();
    let mut encode = vec![];
    for c in start.bytes() {
        encode.push(c ^ key_iter.next().unwrap());
    }
    bytes2hex(&encode)
}

pub fn repeating_key_xor_encode_pretty(start: &str, key: &str) -> String {
    hex2hex_pretty(&repeating_key_xor_encode(start, key))
}
