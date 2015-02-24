extern crate hyper;
use self::hyper::Client;
use single_byte_xor::*;
use utilities::*;
static FILE: &'static str = "http://cryptopals.com/static/challenge-data/4.txt";

fn get_file() -> String {
    Client::new()
    .get(FILE)
    .send().unwrap()
    .read_to_string().unwrap()
}

pub fn detect_single_char_xor_decode() -> String {
    find_best(
        get_file()
        .lines()
        .map(find_xor_byte_decode)
        .collect()
    )
}
