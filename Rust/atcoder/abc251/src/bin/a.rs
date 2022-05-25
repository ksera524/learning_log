use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s:String,
    }
    println!("{}",s.repeat(6 / s.len()));
}
