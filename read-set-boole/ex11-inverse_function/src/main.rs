fn reverse_map(n: f64) -> (u16, u16){
    let max_value: f64 = (1u64 << 32) as f64;
    let interleaved = n * max_value;
    calculate_reverse_interleaved(interleaved)
}




fn main() {
    let space = map(5, 12);
    println!("{space}");
    let (x, y) = reverse_map(space);
    println!("X: {x}\nY: {y}");
}

fn calculate_reverse_interleaved(interleaved: f64) -> (u16, u16){
    let new_interleaved = interleaved as u64;
    let mut x = 0u16;
    let mut y = 0u16;
    
    for i in 0..16{
        let x_i = (new_interleaved >> (2 * i + 1)) & 1;
        let y_i = (new_interleaved >> (2 * i)) & 1;
        x |= (x_i as u16) << i;
        y |= (y_i as u16) << i;
    }
    (x, y)
}

// Curve exercise
fn calculate_interleaved(x: u16, y: u16) -> u64{
    let mut res: u64 = 0;
    let new_x: u64 = x as u64;
    let new_y: u64 = y as u64;
    for i in 0..16 {
        let x_i = (new_x >> i) & 1;
        let y_i = (new_y >> i) & 1;
        res |= y_i << (2 * i);
        res |= x_i << (2 * i + 1);
    }
    res
}

fn map(x: u16, y: u16) -> f64 {
    let interleaved: u64 = calculate_interleaved(x, y);
    let max_value: f64 = (1u64 << 32) as f64;
    interleaved as f64 / max_value
}
