fn multiplier(a: u32, b: u32) -> u32{
    let mut new_a = a;
    let mut new_b = b;
    let mut res = 0u32;

    while new_b != 0 {
        if new_b&1 != 0 {
            res = adder(res, new_a);
        }
        new_a <<= 1;
        new_b >>= 1;
    }
    res
}




fn main() {
    let a = 10u32;
    let b = 30u32;
    println!("NORMAL ARITHMETIC(*): {a} * {b} = {}\n", a * b);
    println!("THE FUNCTION MULTIPLIER: {a} * {b} = {}\n", multiplier(a, b));
}



fn adder(a: u32, b: u32) -> u32{
    if b == 0 {
        a
    }else{
        adder(a ^ b, (a & b) << 1)   
    }
}
