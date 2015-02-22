use utilities::*;

fn single_byte_xor(hex_str: &str, byte: u8) -> String {
    let hex_encode = hex_pretty2hex(hex_str);
    let mut xored_hex = vec![];
    for hex_digits in hex_encode.chunks(2) {
        let higher = hex_digits[0];
        let lower = hex_digits[1];
        let this_byte = (higher << 4) + lower;
        xored_hex.push((byte ^ this_byte) as char);
    }
    xored_hex.into_iter().collect()
}



pub fn find_xor_byte(hex_str: &str) -> String {
    let mut possibilities = vec![];
    for x in 0..255 {
        possibilities.push(single_byte_xor(hex_str, x));
    }
    find_best(possibilities)
}
