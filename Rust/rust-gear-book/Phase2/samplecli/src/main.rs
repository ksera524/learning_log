use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader,stdin};
use anyhow::{bail,ensure,Context,Result};
use std::path::PathBuf;

#[derive(Clap,Debug)]
#[clap(
    name = "RPN Program",
    version = "1.0.0",
    author = "Your name",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    fomula_file: Option<PathBuf>,
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }


    fn eval_inner(&self,tokens:&mut Vec<&str>) -> Result<i32>{ 
        let mut stack = Vec::new();
        let mut pos = 0;

        while let Some(token) = tokens.pop() {
            pos += 1;
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().context(format!("invalied syntax at {}",pos))?;
                let x = stack.pop().context(format!("invalied syntax at {}",pos))?;

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalied token at {}",pos),
                };
                stack.push(res);
            }
            if self.0 {
                println!("{:?} {:?}",tokens,stack);
            }
        }
        ensure!(stack.len() == 1,"invalied syntax");

        Ok(stack[0])
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.fomula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

fn run<R:BufRead>(reader:R,verbose:bool) -> Result<()>  {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line?;
        match calc.eval(&line) {
            Ok(answer) => println!("{}" ,answer),
            Err(e) => println!("{:#?}",e),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let clap = RpnCalculator::new(false);

        assert_eq!(clap.eval("5").unwrap(), 5);
        assert_eq!(clap.eval("50").unwrap(), 50);
        assert_eq!(clap.eval("5 5 *").unwrap(), 25);
        assert_eq!(clap.eval("5 5 +").unwrap(), 10);
        assert_eq!(clap.eval("5 5 -").unwrap(), 0);
        assert_eq!(clap.eval("5 5 /").unwrap(), 1);
        assert_eq!(clap.eval("5 5 %").unwrap(), 0);

        assert_eq!(clap.eval("5 5 + 5 * 10 -").unwrap(), 40);
    }

    #[test]
    fn test_ng() {
        let clap = RpnCalculator::new(false);
        assert!(clap.eval("1 1 ^").is_err());
    }
}