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
                    _ => 0
                })
}

pub fn find_xor_byte(hex_str: &str) -> String {
    let mut possibilities = vec![];
    for x in 0..255 {
        possibilities.push(single_byte_xor(hex_str, x));
    }
    possibilities.sort_by(|x,y| score(x).cmp(&score(y)));
    println!("{}", possibilities.last().unwrap_or(&"".to_string()));
    "".to_string()
}
