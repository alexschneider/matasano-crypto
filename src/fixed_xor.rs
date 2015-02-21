use utilities::*;

pub fn fixed_xor(hex1 : &[u8], hex2 : &[u8]) -> Vec<u8> {
    let bin1 = hex2bin(hex1);
    let bin2 = hex2bin(hex2);
    let mut xor_bin = vec![];
    for (dig1, dig2) in bin1.iter().zip(bin2.iter()) {
        xor_bin.push(dig1 ^ dig2);
    }
    bin2hex(&xor_bin)
}

pub fn fixed_xor_pretty(hex1_pretty: &str, hex2_pretty: &str) -> String {
    let hex1 = hex_pretty2hex(hex1_pretty);
    let hex2 = hex_pretty2hex(hex2_pretty);
    hex2hex_pretty(&fixed_xor(&hex1, &hex2))
}
