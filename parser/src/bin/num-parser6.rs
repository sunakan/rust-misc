extern crate nom;

use nom::{Err, IResult};
use nom::error::ErrorKind;
use nom::number::complete::double;

fn floating_point(s: &str) -> IResult<&str, f64> {
    double(s)
}

fn main() {
    assert_eq!(floating_point("1.234e-8+0.24"), Ok(("+0.24", 1.234e-8)));
    assert_eq!(floating_point("1.234"),         Ok(("", 1.234)));
    assert_eq!(floating_point("1234"),          Ok(("", 1234f64)));
    assert_eq!(floating_point(".234"),          Ok(("", 0.234)));
    assert_eq!(floating_point("EFG"),           Err(Err::Error(("EFG", ErrorKind::Float))));
    assert_eq!(floating_point("ABC"),           Err(Err::Error(("ABC", ErrorKind::Float))));
}
