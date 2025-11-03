
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn exists_in(&self, year: u16) -> bool {
        match self {
            &UsState::Alabama => year >= 1819,
            &UsState::Alaska => year >= 1959,
            // --snip--
        }
    }
}

fn describe_state_quarter(coin: Coin, year: u16) {
    let Coin::Quarter(state) = coin else {
        return;
    };
    if state.exists_in(year) {
        println!("The quarter from {:?} existed in {}.", state, year);
    } else {
        println!("The quarter from {:?} did not exist in {}.", state, year);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn main() {

    let coin1 = Coin::Penny;
    let coin2 = Coin::Dime;
    let coin3 = Coin::Quarter(UsState::Alaska);
    let coin4 = Coin::Nickel;

    println!("Value of coin1: {} cents", value_in_cents(coin1));
    println!("Value of coin2: {} cents", value_in_cents(coin2));
    println!("Value of coin4: {} cents", value_in_cents(coin4));

    describe_state_quarter(coin3, 1960);
    //describe_state_quarter(coin3, 1900)
  
}