use proconio::{fastout, input};
use proconio::marker::*;

#[fastout]
fn main() {
    input! {
        n:i32,
        k:i32,
        a:[usize;n],
        b:[Usize1;k],
    }
    let max = *a.iter().max().unwrap();

    println!("{}", if b.iter().any(|&i| a[i] == max) {"Yes"} else {"No"});
}