extern crate nom;

use std::collections::HashMap;
use nom::IResult;

pub enum JsonValue {
  Str(String),
//  Boolean(bool),
//  Num(f64),
//  Array(Vec<JsonValue>),
//  Object(HashMap<String, JsonValue>),
}

fn parse(s: &str) -> IResult<&str, JsonValue> {
  string()
}

fn main() {
  let data = r#"
  {
    "a"  : -42,
    "b": [ "x", "y", 12 ] ,
    "c": { "hello" : "world", "a" : -3.14 },
    "d"  : true,
    "e"  : [{ "good" : "day" }, -1, 2]
  }"#;
  let data = r#""hello""#;
  println!("{}", data);
}
