extern crate nom;

use nom::{Err, IResult};
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::opt;
use nom::error::ErrorKind;

fn decimal(s: &str) -> IResult<&str, f64> {
    let (s, (sign, int, _, frac)) = nom::branch::permutation((
        opt(one_of("+-")), digit1, char('.'), digit1
    ))(s)?;
    let sign: f64 = if sign.unwrap_or('+') == '+' { 1.0 } else { -1.0 };
    let integer: f64 = int.parse::<f64>().unwrap();
    let fraction: f64 = frac.parse::<f64>().unwrap() / 10f64.powf(frac.len() as f64);
    Ok((s, (integer + fraction).copysign(sign)))
}

fn main() {
    assert_eq!(decimal("3.14"), Ok(("", 3.14)));
    assert_eq!(decimal("+3.14"), Ok(("", 3.14)));
    assert_eq!(decimal("-3.14"), Ok(("", -3.14)));
    assert_eq!(decimal("+3"), Err(Err::Error(("", ErrorKind::Permutation))));
    assert_eq!(decimal(".14"), Err(Err::Error(("", ErrorKind::Permutation))));
}
