#![feature(old_io)]
extern crate matasano;


use matasano::hex2base64::*;
use matasano::fixed_xor::*;
use matasano::single_byte_xor::*;
use std::old_io;
fn main() {
    println!("{}", find_xor_byte("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"))
}
