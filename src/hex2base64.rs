use utilities::*;

fn hex2base64(hex: &[u8]) -> Vec<u8> {
    bin2base64(&hex2bin(hex))
}

pub fn hex2base64_pretty(hex: &str) -> String {
    let hex_digits = hex_pretty2hex(hex);
    let base64 = hex2base64(&hex_digits[..]);
    base64_2_base64_pretty(&base64)
}
