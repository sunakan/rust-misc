extern crate nom;

use nom::{Err, IResult};
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::opt;
use nom::error::ErrorKind;

fn decimal(s: &str) -> IResult<&str, f64> {
    let (s, sign) = opt(one_of("+-"))(s)?;
    let (s, int) = digit1(s)?;
    let (s, _) = char('.')(s)?;
    let (s, frac) = digit1(s)?;

    // fractionの意味：分数
    let sign: f64 = if sign.unwrap_or('+') == '+' { 1.0 } else { -1.0 };
    let integer: f64 = int.parse::<f64>().unwrap();
    let fraction: f64 = frac.parse::<f64>().unwrap() / 10f64.powf(frac.len() as f64);
    Ok((s, (integer + fraction).copysign(sign)))
}

fn main() {
    assert_eq!(decimal("3.14"), Ok(("", 3.14)));
    assert_eq!(decimal("+3.14"), Ok(("", 3.14)));
    assert_eq!(decimal("-3.14"), Ok(("", -3.14)));
    assert_eq!(decimal("+3"), Err(Err::Error(("", ErrorKind::Char))));
    assert_eq!(decimal(".14"), Err(Err::Error((".14", ErrorKind::Digit))));
}
