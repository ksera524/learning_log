use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("数を当ててごらん");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop{
    println!("ほら、予想を入力してね");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Faild to read line");

    let guess: u32 = match  guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("あなたは {} と予想しました",guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("小さすぎ！"),
        Ordering::Greater => println!("大きすぎ！"),
        Ordering::Equal => {
            println!("やったね！");
            break;
            }   
        }
    }
}
