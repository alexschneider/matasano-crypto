static HEX_STRING: &'static str = "0123456789ABCDEF";

pub fn bin2base64(bin: &[u8]) -> Vec<u8> {
    let mut base64_digits = vec![];
    for digit in bin.chunks(6) {
        let mut number = 0;
        for &bin_num in digit.iter() {
            number <<= 1;
            number += bin_num;
        }
        if digit.len() != 6 {
            number <<= 6 - digit.len();
        }
        base64_digits.push(number);
    }
    base64_digits
}

pub fn hex2bin(hex: &[u8]) -> Vec<u8> {
    let mut bin_digits = vec![];
    for digit in hex.iter() {
        let bin = vec![
            (digit >> 3) & 1,
            (digit >> 2) & 1,
            (digit >> 1) & 1,
            digit & 1
        ];
        bin_digits.extend(bin);
    }
    bin_digits
}

pub fn bin2hex(bin: &[u8]) -> Vec<u8> {
    let mut hex_digits = vec![];
    for digits in bin.chunks(4) {
        let mut number = 0;
        for &bin_num in digits.iter() {
            number <<= 1;
            number += bin_num;
        }
        if digits.len() != 4 {
            number <<= 4 - digits.len();
        }
        hex_digits.push(number);
    }
    hex_digits
}

pub fn hex_pretty2hex(hex: &str) -> Vec<u8> {
    let mut hex_digits = vec![];
    for c in hex.chars() {
        let digit = HEX_STRING.find(c.to_uppercase())
                          .expect("Invalid hex string") as u8;
        hex_digits.push(digit);
    }
    hex_digits
}

pub fn hex2hex_pretty(hex: &[u8]) -> String {
    let mut hex_chars = vec![];
    for d in hex.iter() {
        hex_chars.push(HEX_STRING.char_at(*d as usize));
    }
    hex_chars.into_iter().collect()
}

pub fn base64_2_base64_pretty(base64: &[u8]) -> String {

    let pretty_base64 = base64.iter().map(|&x| match x {
        0...25 => (('A' as u8) + x) as char,
        26...51 => (('a' as u8) + x - 26) as char,
        52...61 => (('0' as u8) + (x - 52)) as char,
        62 => '+',
        63 => '/',
        _ => panic!("Invalid base64 number")
    });
    pretty_base64.collect()
}

fn score(score_string: &str) -> usize {
    // From http://en.wikipedia.org/wiki/Letter_frequency
    score_string.chars()
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
    let mut possibilities = strings.clone();
    possibilities.sort_by(|x,y| score(x).cmp(&score(y)));
    possibilities.last().unwrap().clone()
}
