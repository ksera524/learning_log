extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

type Record = (String,String,i32);

fn get_first_args() -> OsString{
    match env::args_os().nth(1) {
        None => From::from("引数ないよ"),
        Some(file_path) => file_path, 
    }
}

fn run() ->  Result<(), Box<dyn Error>>{
    let file_path = get_first_args();
    let mut rdr = csv::Reader::from_path(file_path)?;
    rdr.deserialize().for_each(|result| {
        let record:Record = result?;
        println!("{:?}",record);
    });
    Ok(())
}

fn main() {
    if let Err(err) = run(){
        print!("{}",err);
        process::exit(1);
    }
}
