struct color {
    r: i32,
    g: i32,
    b: i32,
}

fn main() {
    let red = color { r: 255, g: 0, b: 0 };
    let green = color { r: 0, g: 255, b: 0 };
    let blue = color { r: 0, g: 0, b: 255 };

    println!("Red:   r={}, g={}, b={}", red.r, red.g, red.b);
    println!("Green: r={}, g={}, b={}", green.r, green.g, green.b);
    println!("Blue:  r={}, g={}, b={}", blue.r, blue.g, blue.b);
}
