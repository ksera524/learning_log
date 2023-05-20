fn main() {
    println!("Hello, world!");

    //符号ビットの取り出し
    let n: f32 = -42.42;
    let n_bits: u32 = n.to_bits();
    println!("n_bits: {:32b}", n_bits);
    let sign_bit = n_bits >> 31; 
    println!("sign_bit: {:32b}", sign_bit);

    //指数部の取り出し
    let n: f32 = 42.42; 
    let n_bits: u32 = n.to_bits(); 
    println!("n_bits   : {:32b}", n_bits);
    let exponent_ = n_bits >> 23; 
    println!("exponent_: {:32b}", exponent_);
    println!("&0xff    : {:32b}", &0xff);
    let exponent_ = exponent_ &0xff;
    println!("exponent_: {:32b}", exponent_);
    let exponent = (exponent_ as i32) - 127;
    println!("exponent: {}", exponent);

    //仮数部の取り出し1
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    println!("n_bits:   {:32b}", n_bits);
    let mantissa = n_bits & 0x7fffff;
    println!("mantissa: {:32b}", mantissa);
    let mantissa = mantissa | 0x800000;
    println!("mantissa: {:32b}", mantissa);
    let mantissa = mantissa as f32 / 0x800000 as f32;
    println!("mantissa: {}", mantissa);

    //仮数部の取り出し2
    let n: f32 = 42.42;
    let n_bits: u32 = n.to_bits();
    println!("n_bits:   {:32b}", n_bits);
    let mut mantissa:f32 = 1.0;
    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit_i = n_bits & mask;
        println!("{} mask:   {:32b}", i,mask);
        println!("{} one_i:   {:32b}", i,one_at_bit_i);
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            println!("i:{}", i_);
            mantissa += 2.0_f32.powf(i_ - 23.0);
        }
    }
    println!("mantissa: {}", mantissa);

}