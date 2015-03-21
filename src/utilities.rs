use std::ascii::AsciiExt;
use std::num::Int;
static HEX_STRING: &'static str = "0123456789ABCDEF";

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hamming_distance() {
        assert_eq!(
            hamming_distance(b"this is a test", 
                             b"wokka wokka!!!"
            ), 
            37
        )
    }
}

pub fn hamming_distance(string1: &[u8], string2: &[u8]) -> u32 {
    string1.iter().zip(string2.iter())
    .fold(0u32, 
          |acc, (char1, char2)| acc + (char1 ^ char2).count_ones()
    )
}

pub fn bin2base64(bin: Vec<u8>) -> Vec<u8> {
    bin
    .chunks(6)
    .map(|digit| {
        digit
        .iter()
        .fold(0, |number, &dig| (number << 1) + dig)
        as u8
    })
    .collect()
}

pub fn hex2bin(hex: Vec<u8>) -> Vec<u8> {
    hex
    .iter()
    .flat_map(|digit| vec![
        (digit >> 3) & 1,
        (digit >> 2) & 1,
        (digit >> 1) & 1,
        digit & 1
    ].into_iter())
    .collect()
}

pub fn hex2bytes(hex: Vec<u8>) -> Vec<u8> {
    hex.
    chunks(2)
    .map(|digits|
        if digits.len() == 2 {
            (digits[0] << 4) + digits[1]
        } else {
            digits[0] << 4
        }
    )
    .collect()
}

pub fn bytes2hex(bytes: Vec<u8>) -> Vec<u8> {
     bytes
    .iter()
    .enumerate()
    .filter_map(|(pos, &byte)|
        if pos == bytes.len() - 1 && byte == 0 {
            None
        } else {
            Some(
                vec![(byte >> 4) & 0xF, byte & 0xF].into_iter()
            )
        }
    )
    .flat_map(|x| x)
    .collect()
}

pub fn bin2hex(bin: Vec<u8>) -> Vec<u8> {
    bin
    .chunks(4)
    .map(|d| d.iter().fold(0, |number, &digit| (number << 1) + digit))
    .collect()
}

pub fn hex_pretty2hex(hex: &str) -> Vec<u8> {
    hex
    .to_ascii_uppercase()
    .chars()
    .map(|c|
        HEX_STRING
        .find(c)
        .expect("Invalid hex string")
        as u8
    )
    .collect()
}

pub fn hex2hex_pretty(hex: Vec<u8>) -> String {
    hex
    .iter()
    .map(|x| HEX_STRING.char_at(*x as usize))
    .collect()
}

pub fn base64_2_base64_pretty(base64: Vec<u8>) -> String {
    base64
    .iter()
    .map(|&x| match x {
        0...25 => (('A' as u8) + x) as char,
        26...51 => (('a' as u8) + x - 26) as char,
        52...61 => (('0' as u8) + (x - 52)) as char,
        62 => '+',
        63 => '/',
        _ => panic!("Invalid base64 number")
    })
    .collect()
}

fn score(score_string: &&String) -> usize {
    // From http://en.wikipedia.org/wiki/Letter_frequency
    score_string
    .chars()
    .fold(0, |a, b| a + match b {
        'a' => 8167,
        'b' => 1492,
        'c' => 2782,
        'd' => 4253,
        'e' => 12702,
        'f' => 2228,
        'g' => 2015,
        'h' => 6094,
        'i' => 6966,
        'j' => 0153,
        'k' => 0772,
        'l' => 4025,
        'm' => 2406,
        'n' => 6749,
        'o' => 7507,
        'p' => 1929,
        'q' => 0095,
        'r' => 5987,
        's' => 6327,
        't' => 9056,
        'u' => 2758,
        'v' => 0978,
        'w' => 2360,
        'x' => 0150,
        'y' => 1974,
        'z' => 0074,
        ' ' => 2000,
        _ => 0
    })
}

pub fn find_best(strings: Vec<String>) -> String {
    strings
    .iter()
    .max_by(score)
    .unwrap()
    .clone()
}

