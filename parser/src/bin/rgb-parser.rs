extern crate nom;

use nom::error::ErrorKind;
use nom::character::complete::char;
use nom::{IResult,Err};
use nom::combinator::{map_res, map};
use nom::bytes::complete::take_while_m_n;
use nom::character::is_hex_digit;
use nom::sequence::preceded;
use nom::multi::count;

fn hex_to_u8(s: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, |ch| is_hex_digit(ch as u8)),
        |x| u8::from_str_radix(x, 16),
    )(s)
}

fn rbg(s: &str) -> IResult<&str, u32> {
    preceded(
        char('#'),
        map(
            count(hex_to_u8, 3),
            |hex| (hex[0] as u32) << 16 | (hex[1] as u32) << 8 | hex[2] as u32,
        ),
    )(s)
}

fn main() {
    assert_eq!(rbg("#000000"), Ok(("", 0)));
    assert_eq!(rbg("#FFFFFF"), Ok(("", 0xFFFFFF)));
    assert_eq!(rbg("#7F0000"), Ok(("", 0x7F0000)));
    assert_eq!(rbg("#007F00"), Ok(("", 0x007F00)));
    assert_eq!(rbg("#00007F"), Ok(("", 0x00007F)));
    assert_eq!(rbg("$FFFFFF"), Err(Err::Error(("$FFFFFF", ErrorKind::Char))));
    assert_eq!(rbg("#FFFGHI"), Err(Err::Error(("FGHI", ErrorKind::TakeWhileMN))));
    assert_eq!(rbg("#FF"), Err(Err::Error(("", ErrorKind::TakeWhileMN))));
}
