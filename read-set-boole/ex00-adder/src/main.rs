fn adder(a: u32, b: u32) -> u32{
    if b == 0 {
        a
    }else{
        adder(a ^ b, (a & b) << 1)
    }
    
}




fn main() {
    let a = 10u32;
    let b = 30u32;
    println!("NORMAL ARITHMETIC(+): {a} + {b} = {}\n", a + b);
    println!("JUST THE XOR BITWISE(^): {a} + {b} = {}\n", a ^ b);
    println!("THE FUNCTION ADDER(^ WITH CARRY): {a} + {b} = {}\n", adder(a, b));
}
