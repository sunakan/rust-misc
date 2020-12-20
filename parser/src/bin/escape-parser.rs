extern crate nom;

use nom::{Err, IResult};
use nom::branch::{alt, permutation};
use nom::sequence::delimited;
use nom::bytes::complete::{escaped_transform, take_while_m_n};
use nom::character::complete::{char, none_of};
use nom::combinator::{value, map};
use nom::error::ErrorKind;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::u16;

fn string_literal(s: &str) -> IResult<&str, String> {
    delimited(
        char('\"'),
        escaped_transform(none_of("\"\\"), '\\', alt((
            value('\\', char('\\')),
            value('\"', char('\"')),
            value('\'', char('\'')),
            value('\r', char('r')),
            value('\n', char('n')),
            value('\t', char('t')),
            map(
                permutation((char('u'), take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()))),
                |(_, code): (char, &str)| -> char {
                    decode_utf16(vec![u16::from_str_radix(code, 16).unwrap()])
                        .nth(0).unwrap().unwrap_or(REPLACEMENT_CHARACTER)
                },
            ),
        ))),
        char('\"')
    )(s)
}

fn main() {
    assert_eq!(string_literal("\"a\\\"b\\\'c\""), Ok(("", String::from("a\"b\'c"))));
    assert_eq!(string_literal("\" \\r\\n\\t \\u2615 \\uDD1E\""), Ok(("", String::from(" \r\n\t ☕ �"))));
    assert_eq!(string_literal("\"abc"), Err(Err::Error(("", ErrorKind::Char))));
    assert_eq!(string_literal("\"ab\\zc\""), Err(Err::Error(("zc\"", ErrorKind::Permutation))));
}
