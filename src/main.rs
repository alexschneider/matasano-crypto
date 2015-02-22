#![feature(old_io)]
extern crate matasano;


use matasano::hex2base64::*;
use matasano::fixed_xor::*;
use matasano::single_byte_xor::*;
use matasano::repeating_key_xor_encode::*;
use std::old_io;
fn main() {
    let message = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    println!("{}", repeating_key_xor_encode_pretty(message, "ICE"));

}
