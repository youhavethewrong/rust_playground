use rug::Float;

fn main() {
    let i = 9999999999999999.0;
    let j = 9999999999999998.0;
    let k = i - j;
    println!("float 64: {}", k);

    let l = Float::with_val(200, Float::parse("9999999999999999.0").unwrap());
    let m = Float::with_val(200, Float::parse("9999999999999998.0").unwrap());
    let n = l - m;
    println!("rug Float: {}", n);
}
