fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let sum = x + y as f64;

    // default formatting
    println!("x + y = {}", sum);

    // formatted to 2 decimal places
    println!("x + y (2 decimals) = {:.2}", sum);
}
