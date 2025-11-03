fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is: {} square pixels.", 
                area1(width1, height1));
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

