use std::option::Option;

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Option::Some(value + 1),
        None => Option::None,
        _ => x
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let some_str = Some("Hello, world!");

    println!("six: {:?}", six);
    println!("none: {:?}", none);
    println!("some_str: {:?}", some_str);
}