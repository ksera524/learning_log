use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n:u32,
        a:[(String,u32);n],
    }

    let mut s =HashSet::new();
    let mut ans = 0;
    let mut score = 0;

    for(k ,(i,j) ) in a.into_iter().enumerate() {
        if !s.contains(&i){
            s.insert(i);
            if j > score {
                score = j;
                ans = k + 1;
            }
        }
    }
    println!("{}",ans);
}
