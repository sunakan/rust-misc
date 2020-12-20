extern crate nom;
use nom::character::complete::{char, digit1, multispace0};
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::{Err, IResult};

fn numeric_in_parentheses(s: &str) -> IResult<&str, &str> {
    delimited(
        char('('),
        delimited(multispace0, digit1, multispace0),
        char(')'),
    )(s)
}

fn main() {
    assert_eq!(numeric_in_parentheses("(0)"), Ok(("", "0")));
    assert_eq!(numeric_in_parentheses("( 123 )"), Ok(("", "123")));
    assert_eq!(numeric_in_parentheses("(123)456"), Ok(("456", "123")));
    assert_eq!(
        numeric_in_parentheses("1234"),
        Err(Err::Error(("1234", ErrorKind::Char)))
    );
    assert_eq!(
        numeric_in_parentheses("()"),
        Err(Err::Error((")", ErrorKind::Digit)))
    );
}
