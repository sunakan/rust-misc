extern crate nom;

use nom::{Err, IResult};
use nom::branch::alt;
use nom::character::complete::{char, digit1, multispace0};
use nom::error::ErrorKind;
use nom::sequence::delimited;

fn numeric_in_parentheses(s: &str) -> IResult<&str, &str> {
    let (s, open) = alt((
        char('('), char('{'), char('['),
    ))(s)?;
    let close = match open {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        _ => panic!(),
    };
    let (s, num) = delimited(multispace0, digit1, multispace0)(s)?;
    let (s, _) = char(close)(s)?;
    Ok((s, num))
}

fn main() {
    assert_eq!(numeric_in_parentheses("(0)"),      Ok(("", "0")));
    assert_eq!(numeric_in_parentheses("( 123 )"),  Ok(("", "123")));
    assert_eq!(numeric_in_parentheses("{ 456 }"),  Ok(("", "456")));
    assert_eq!(numeric_in_parentheses("[ 789 ]"),  Ok(("", "789")));
    assert_eq!(numeric_in_parentheses("(123)456"), Ok(("456", "123")));
    assert_eq!(numeric_in_parentheses("1234"),     Err(Err::Error(("1234", ErrorKind::Char))));
    assert_eq!(numeric_in_parentheses("()"),       Err(Err::Error((")", ErrorKind::Digit))));
    assert_eq!(numeric_in_parentheses("<999>"),    Err(Err::Error(("<999>", ErrorKind::Char))));
    assert_eq!(numeric_in_parentheses("(234}"),    Err(Err::Error(("}", ErrorKind::Char))));
}
