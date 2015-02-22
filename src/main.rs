#![feature(old_io)]
extern crate matasano;


use matasano::hex2base64::*;
use matasano::fixed_xor::*;
use matasano::single_byte_xor::*;
use matasano::detect_single_char_xor::*;
use std::old_io;
fn main() {
    println!("{}", detect_single_char_xor())
}
