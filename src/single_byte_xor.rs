use utilities::*;

pub fn single_byte_xor(hex_str: &str, byte: char) -> String {
    let hex_encode = hex_pretty2hex(hex_str);
    let xored_hex = hex_encode.iter().map(|x| (x ^ (byte as u8)) as char);
    xored_hex.collect()
}
