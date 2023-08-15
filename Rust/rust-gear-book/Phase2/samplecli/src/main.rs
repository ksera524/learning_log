use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader,stdin};

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
    fomula_file: Option<String>,
}

struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }


    fn eval_inner(&self,tokens:&mut Vec<&str>) ->  i32{ 
        let mut stack = Vec::new();

        while let Some(token) = tokens.pop() {
            if let Ok(x) = token.parse::<i32>() {
                stack.push(x);
            } else {
                let y = stack.pop().expect("invalied syntax");
                let x = stack.pop().expect("invalied syntax");

                let res = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => panic!("invalied token"),
                };
                stack.push(res);
            }
            if self.0 {
                println!("{:?} {:?}",tokens,stack);
            }
        }
        if stack.len() == 1 {
            stack[0]
        } else {
            panic!("invalied syntax")
        }
    }
}

fn main() {
    let opts = Opts::parse();

    if let Some(path) = opts.fomula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose);
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose);
    }
}

fn run<R:BufRead>(reader:R,verbose:bool)  {
    let calc = RpnCalculator::new(verbose);
    for line in reader.lines() {
        let line = line.unwrap();
        let answer = calc.eval(&line);
        println!("{}", answer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let clap = RpnCalculator::new(false);

        assert_eq!(clap.eval("5"), 5);
        assert_eq!(clap.eval("50"), 50);
        assert_eq!(clap.eval("5 5 *"), 25);
        assert_eq!(clap.eval("5 5 +"), 10);
        assert_eq!(clap.eval("5 5 -"), 0);
        assert_eq!(clap.eval("5 5 /"), 1);
        assert_eq!(clap.eval("5 5 %"), 0);

        assert_eq!(clap.eval("5 5 + 5 * 10 -"), 40);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let clap = RpnCalculator::new(false);
        clap.eval("1 1 ^");
    }
}