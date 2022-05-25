use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n:usize,
        w:u32,
        a:[u32;n],
    }
    
    let mut s =HashSet::new();

    for i in 0..n {
        let sum = a[i];
        if sum <= w {
            s.insert(sum);
        }
        for j in i+1 ..n{
            let sum = a[i] + a[j];
            if sum <= w {
                s.insert(sum);
            }for k in j+1..n{
                let sum = a[i] + a[j] + a[k];
                if sum <= w{
                    s.insert(sum);
                }
            }
        }
    }
    println!("{}",s.len());
}
