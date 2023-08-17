use anyhow::{Context,Result,bail,ensure};

fn get_int_from_file() -> Result<i32> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).with_context(||format!("faild to read string from {}",path))?;

    if num_str.len() > 10 {
        bail!("it may be too large number");
    }

    ensure!(num_str.starts_with("1"), "first digit is not 1");

    num_str.trim().parse::<i32>().map(|t|t * 2).context("failed to parse String")
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}",x),
        Err(e) => println!("{}",e),
    }
}