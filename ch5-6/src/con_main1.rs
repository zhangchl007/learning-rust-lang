use std::option::Option;

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(value) => Option::Some(value + 1),
        None => Option::None,
        _ => x
    }
}

fn main() {
    let some_str = Some("Hello, world!");

    if some_str.is_some() {
        println!("some_str has a value: {:?}", some_str);
    } else {
        println!("some_str is None");
    }
}