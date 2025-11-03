
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Dime;
    let coin3 = Coin::Quarter;
    let coin4 = Coin::Nickel;

    println!("Value of coin1: {} cents", value_in_cents(coin1));
    println!("Value of coin2: {} cents", value_in_cents(coin2));
    println!("Value of coin3: {} cents", value_in_cents(coin3));
    println!("Value of coin4: {} cents", value_in_cents(coin4));
}   