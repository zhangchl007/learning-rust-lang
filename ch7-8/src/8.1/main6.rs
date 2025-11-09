use std::vec;

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    /*let first = v.first();
    print!("{:?}\n", first);*/
    let first = &v[0];
    v.push(6);
    print!("{:?}\n", first);
   
}
