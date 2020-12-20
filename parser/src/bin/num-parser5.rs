extern crate nom;

use nom::{Err, IResult};
use nom::branch::permutation;
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::{map_res, opt};
use nom::error::ErrorKind;
use std::num::ParseFloatError;

fn floating_point(s: &str) -> IResult<&str, f64> {
    map_res(
        permutation((opt(one_of("+-")), digit1, char('.'), digit1)),
        | (sign, int, _, frac): (Option<char>, &str, char, &str)| -> Result<f64, ParseFloatError> {
            let sign: f64 = if sign.unwrap_or('+') == '+' { 1.0 } else { -1.0 };
            let integer: f64 = int.parse::<f64>()?;
            let fraction: f64 = frac.parse::<f64>()? / 10f64.powf(frac.len() as f64);
            Ok((integer + fraction).copysign(sign))
        },
    )(s)
}

fn main() {
    assert_eq!(floating_point("3.14"),  Ok(("", 3.14)));
    assert_eq!(floating_point("+3.14"), Ok(("", 3.14)));
    assert_eq!(floating_point("-3.14"), Ok(("", -3.14)));
    assert_eq!(floating_point("+3"),    Err(Err::Error(("", ErrorKind::Permutation))));
    assert_eq!(floating_point(".14"),   Err(Err::Error(("", ErrorKind::Permutation))));
}
