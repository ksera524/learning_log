use std::fmt;

enum MyError {
    Io(std::io::Error),
    Num(std::num::ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f,"I/O Error: {}", cause ),
            MyError::Num(cause) => write!(f,"Parse Error: {}", cause),
        }
    }
}

fn get_int_from_file() -> Result<i32,MyError> {
    let path = "number.txt";
    let num_str = std::fs::read_to_string(path).map_err(|e|MyError::Io(e))?;

    num_str
        .trim()
        .parse::<i32>()
        .map(|t| t * 2)
        .map_err(|e|MyError::Num(e))
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}",e),
    }
}
