const BIAS:i32 = 127;
const RADIX:f32 = 2.0;

fn main(){
    let n:f32 = 42.42;

    let (sign,exp,frac) = to_parts(n);
    let (sign_,exp_,mant) = decode(sign,exp,frac);
    let n_ = from_parts(sign_,exp_,mant);

    println!("{} -> {}",n,n_);
    println!("フィールド| ビット | 実数");
    println!("符号 | {:01b} | {}",sign,sign_);
    println!("指数部 | {:08b} | {}",exp,exp_);
    println!("仮数部 | {:023b} | {}",frac,mant);
}

fn to_parts(n:f32) -> (u32,u32,u32){
    let bits = n.to_bits();

    let sign = (bits >> 31) & 1;
    let expoint = (bits >> 23) & 0xff;
    let fraction = bits & 0x7fffff;

    (sign,expoint,fraction)
}

fn decode(sign:u32,expoint:u32,fraction:u32) -> (f32,f32,f32){
    let signned_1 = (-1.0_f32).powf(sign as f32);
    let mut mantissa:f32 = 1.0;
    let expoint = (expoint as i32) - BIAS;
    let expoint = RADIX.powf(expoint as f32);


    for i in 0..23{
        let mask = 1 << i;
        let one_at_bit_i = fraction & mask;
        if one_at_bit_i != 0 {
            let i_ = i as f32;
            let weight = 2_f32.powf(i_ - 23.0);
            mantissa += weight;
        }
    }
    (signned_1,expoint,mantissa)
}

fn from_parts(sign:f32,expoint:f32,mantissa:f32) -> f32{
    sign * expoint * mantissa
}