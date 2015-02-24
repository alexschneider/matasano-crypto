use utilities::*;

fn single_byte_xor_decode(hex_str: &str, byte: u8) -> String {
    hex2bytes(hex_pretty2hex(hex_str))
    .iter()
    .map(|&x| (byte ^ x) as char)
    .collect()
}


pub fn find_xor_byte_decode(hex_str: &str) -> String {
    find_best(
        (0..256u16)
        .map(|x|
            single_byte_xor_decode(hex_str, x as u8)
        )
        .collect()
    )
}
