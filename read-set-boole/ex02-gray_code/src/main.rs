fn gray_code(n: u32) -> u32{
    n ^ (n >> 1)
}



fn main() {
    println!("Result: {}\nExpected: 0", gray_code(0));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 1", gray_code(1));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 3", gray_code(2));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 2", gray_code(3));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 6", gray_code(4));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 7", gray_code(5));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 5", gray_code(6));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 4", gray_code(7));
    println!("\n========================\n");
    println!("Result: {}\nExpected: 12", gray_code(8));
    println!("\n========================\n");
}
