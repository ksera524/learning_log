mod engine;
mod helper;

use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead,BufReader},
};

fn main() -> Result<(),DynError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("usage: {} regex file",args[0]);
        return  Err("invalied arguments".into());
    } else {
        match_file(&args[1],&args[2])?;
    }
    Ok(())
}

fn match_file(expr:&str,file: &str) -> Result<(),DynError>{
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    engine::print(expr)?;
    println!();
    for line in reader.lines(){
        let line = line?;
        for (i,_) in line.char_indices(){
            if engine::do_matching(expr, &line[i..], true)?{
                println!("{line}");
                break;
            }
        }
    }
    Ok(())
}