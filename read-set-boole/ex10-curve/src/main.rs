/*for the z-order curve interleaved x and y
example: x = 5 = 0101
         y = 12 = 1100
         interleaved = 114 = 01110010
then normalize interleaved
         ((2^16) - 1)^2 = 4294836225
         interleaved / 4294836225
*/
fn map(x: u16, y: u16) -> f64 {
    let interleaved: u64 = calculate_interleaved(x, y);
    let max_value: f64 = (1u64 << 32) as f64;
    interleaved as f64 / max_value
}


fn main() {
    println!("{}",map(5, 12));
}

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
