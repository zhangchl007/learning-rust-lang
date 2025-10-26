fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 "hello"
    // s.clear(); // 这一行会报错，因为我们不能在有不可变引用时修改 s
    println!("the first word is: {}", word);
}

