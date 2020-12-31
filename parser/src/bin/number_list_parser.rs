extern crate nom;

use nom::character::complete::multispace0;
use nom::character::streaming::digit1;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;

fn number_list(s: &str) -> IResult<&str, Vec<u32>> {
    many0(map_res(
        delimited(multispace0, digit1, multispace0),
        |x: &str| x.parse::<u32>(),
    ))(s)
}

fn main() {
    assert_eq!(number_list("1 2 3 4"), Ok(("", vec![1, 2, 3, 4])));
}
