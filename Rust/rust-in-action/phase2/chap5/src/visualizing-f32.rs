const BIAS:i32 = 127;
const RADIX:f32 = 2.0;

fn main() {
    let n: f32 = -42.42;
    let (sign, exp, frac) = to_parts(n);
    let (sign_, exp_, mant) = decode(sign, exp, frac);
    let n_ = from_parts(sign_, exp_, mant);

    println!("{} -> {}", n,n_);
    println!("符号  |{:01b} |{}",sign, sign_);
    println!("指数  |{:08b} |{}",exp, exp_);
    println!("仮数  |{:023b} |{}",frac, mant);
}

fn to_parts(n:f32) -> (u32, u32, u32) {
    let n_bits: u32 = n.to_bits();
    let sign_bit = n_bits >> 31 & 1;
    let exponent = (n_bits >> 23) & 0xff;
    let fraction = n_bits & 0x7fffff;
    (sign_bit, exponent, fraction)
}

fn decode(sign_bit:u32, exponent:u32, fraction:u32) -> (f32, f32, f32) {
    let sign = if sign_bit == 0 { 1.0 } else { -1.0 };
    let exponent = exponent as i32 - BIAS;
    let exponent = RADIX.powf(exponent as f32);

    let mut mantissa:f32 = 1.0;
    
    for i in 0..23{
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            mantissa += 2.0_f32.powf(i_ - 23.0);
        }
    }
    (sign, exponent, mantissa)
}

fn from_parts(sign:f32, exponent:f32, mantissa:f32) -> f32 {
    sign * exponent * mantissa
}

