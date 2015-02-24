use utilities::*;

pub fn fixed_xor(hex1 : Vec<u8>, hex2 : Vec<u8>) -> Vec<u8> {
    bin2hex(
        hex2bin(hex1)
        .iter()
        .zip(
            hex2bin(hex2)
            .iter()
        )
        .map(|(dig1, dig2)| dig1 ^ dig2)
        .collect()
    )

}

pub fn fixed_xor_pretty(hex1_pretty: &str, hex2_pretty: &str) -> String {
    hex2hex_pretty(
        fixed_xor(
            hex_pretty2hex(hex1_pretty),
            hex_pretty2hex(hex2_pretty)
        )
    )
}
