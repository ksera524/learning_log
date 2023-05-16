use std::time::{Duration,Instant};


fn main() {
    let abc:(f32,f32,f32) = (0.1,0.2,0.3);
    let xyz:(f64,f64,f64) = (0.1,0.2,0.3);

    println!("abc(f32)");
    println!("0.1 + 0.2: {:x}",(abc.0 + abc.1).to_bits());
    println!("0.3 : {:x}",abc.2.to_bits());

    println!("xyz(f64)");
    println!("0.1 + 0.2: {:x}",(xyz.0 + xyz.1).to_bits());
    println!("0.3 : {:x}",xyz.2.to_bits());

    let mut count = 1;
    let time_limit = Duration::new(1,0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}",count);

    let a = 42;
    let b= &a;
    let r = a + *b;
    println!("{}",r);

}