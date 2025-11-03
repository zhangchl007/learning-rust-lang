#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}   

fn main() {

    let mut x: Option<i32> = Option::Some(5);
    let y: Option<i32> = Option::None;

    match x {
        Option::Some(value) => println!("x has value: {}", value),
        Option::None => println!("x is None"),
    }

    match y {
        Option::Some(value) => println!("y has value: {}", value),
        Option::None => println!("y is None"),
    }

    let z: Option<i32> = Option::Some(10);
    if let Option::Some(value) = z {
        x = Option::Some(value + 5);
        println!("Updated x to: {:?}", x);
    }
    
}