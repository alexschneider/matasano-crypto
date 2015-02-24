use utilities::*;

fn hex2base64(hex: Vec<u8>) -> Vec<u8> {
    bin2base64(hex2bin(hex))
}

pub fn hex2base64_pretty(hex: &str) -> String {
    base64_2_base64_pretty(
        hex2base64(
            hex_pretty2hex(hex)
        )
    )
}
