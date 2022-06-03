use proconio::{fastout, input};
use proconio::marker::*;

#[fastout]
fn main() {
    input! {
        a:u8,
        b:u8,
        c:u8
    }
    let mut ans = false;
    if a <= b && b <= c {
        ans = true
    }
    if b <= a && c <= b{
        ans = true
    }

    println!("{}", if ans {"Yes"} else {"No"} );
}
