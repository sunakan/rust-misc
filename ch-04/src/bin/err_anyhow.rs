// 実行方法
// cargo run --bin err_anyhow
use anyhow::{bail, ensure, Context, Result};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read string from {}", path))?;
    if num_str.len() >= 4 {
        bail!("数値が大きすぎるよ");
    }

    ensure!(num_str.starts_with("1"), "最初は1で始まるべき");

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t*2)
        .context("Stringのパースに失敗")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{:#?}", e),
    }
}
