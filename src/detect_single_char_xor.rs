extern crate hyper;
use self::hyper::Client;
use single_byte_xor::*;
use utilities::*;
static FILE: &'static str = "http://cryptopals.com/static/challenge-data/4.txt";

fn get_file() -> String {
    let mut client = Client::new();
    let mut res = client.get(FILE).send().unwrap();
    res.read_to_string().unwrap()
}

pub fn detect_single_char_xor_decode() -> String {
    let mut possibilities = vec![];
    for line in get_file().lines() {
        possibilities.push(find_xor_byte_decode(line));
    }
    find_best(possibilities)
}
