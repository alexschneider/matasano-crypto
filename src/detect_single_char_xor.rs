extern crate hyper;
use self::hyper::Client;
use single_byte_xor::*;
use utilities::*;
use std::io::Read;

static FILE: &'static str = "http://cryptopals.com/static/challenge-data/4.txt";

fn get_file() -> String {
    let mut res = "".to_string();
    Client::new()
    .get(FILE)
    .send().unwrap()
    .read_to_string(&mut res).unwrap();
    res
}

pub fn detect_single_char_xor_decode() -> String {
    find_best(
        get_file()
        .lines()
        .map(find_xor_byte_decode)
        .collect()
    )
}
