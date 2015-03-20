extern crate matasano;
use std::ascii::AsciiExt;
use matasano::*;

#[test]
fn challenge_1() {
    assert_eq!(
        hex2base64::hex2base64_pretty(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69\
            736f6e6f7573206d757368726f6f6d"
        ),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    )
}

#[test]
fn challenge_2() {
    assert!(fixed_xor::fixed_xor_pretty(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965"
        ).eq_ignore_ascii_case(
            "746865206b696420646f6e277420706c6179"
        ))
}

#[test]
fn challenge_3() {
    assert_eq!(
        single_byte_xor::find_xor_byte_decode(
            "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
        ),
        "Cooking MC\'s like a pound of bacon"
    )
}

#[test]
fn challenge_4() {
    assert_eq!(
        detect_single_char_xor::detect_single_char_xor_decode(),
        "Now that the party is jumping\n"
    )
}

#[test]
fn challenge_5() {
    assert!(
        repeating_key_xor_encode::repeating_key_xor_encode_pretty(
                "Burning 'em, if you ain't quick and nimble\n\
                 I go crazy when I hear a cymbal",
                "ICE"
        ).eq_ignore_ascii_case(
            "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324\
             272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165\
             286326302e27282f"
        )

    )
}

#[test]
fn challenge_6() {
    repeating_key_xor_break::break_single_char_xor();
}
