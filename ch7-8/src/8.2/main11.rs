
fn main() {
    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{}", c);
    }
    for b in hello.bytes() {
        println!("{}", b);
    }
    let answer = &hello[0..4];

}
