use std::error::Error;
use reqwest;

fn main() -> Result<(), Box<dyn Error>>{
    let url = "http://google.com/";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    println!("Content:\n\n{}", content);

    Ok(())
}
