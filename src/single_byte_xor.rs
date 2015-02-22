use utilities::*;

fn single_byte_xor_decode(hex_str: &str, byte: u8) -> String {
    let hex_encode = hex_pretty2hex(hex_str);
    let mut xored_hex = vec![];
    for this_byte in hex2bytes(&hex_encode) {
        xored_hex.push((byte ^ this_byte) as char);
    }
    xored_hex.into_iter().collect()
}


pub fn find_xor_byte_decode(hex_str: &str) -> String {
    let mut possibilities = vec![];
    for x in 0..255 {
        possibilities.push(single_byte_xor_decode(hex_str, x));
    }
    find_best(possibilities)
}
