extern crate nom;

use nom::branch::permutation;
use nom::character::complete::{alpha1, char, multispace0};
use nom::error::ErrorKind;
use nom::multi::separated_list;
use nom::sequence::delimited;
use nom::{Err, IResult};

fn list(s: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        char('['),
        delimited(
            multispace0,
            separated_list(permutation((multispace0, char(','), multispace0)), alpha1),
            multispace0,
        ),
        char(']'),
    )(s)
}

fn main() {
    assert_eq!(list("[]"), Ok(("", vec!())));
    assert_eq!(list("[AB, C, DEF]"), Ok(("", vec!("AB", "C", "DEF"))));
    assert_eq!(
        list("AB,C,DEF"),
        Err(Err::Error(("AB,C,DEF", ErrorKind::Char)))
    );
    assert_eq!(list("[A,]"), Err(Err::Error((",]", ErrorKind::Char))));
    assert_eq!(list("[A,,BC]"), Err(Err::Error((",,BC]", ErrorKind::Char))));
    assert_eq!(
        list("[AB, 2, DEF]"),
        Err(Err::Error((", 2, DEF]", ErrorKind::Char)))
    );
    assert_eq!(
        list("[AB, C; DEF]"),
        Err(Err::Error(("; DEF]", ErrorKind::Char)))
    );
    assert_eq!(
        list("[AB, C, DEF)"),
        Err(Err::Error((")", ErrorKind::Char)))
    );
}
