// Rust’s enums are most similar to algebraic data types in functional
// languages, such as F#, OCaml, and Haskell.

// An enum that uses the match control flow operator:

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
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let c = value_in_cents(Coin::Penny);

    println!("c = {} cents", c);
}
