#![allow(dead_code)]
#![allow(unused_variables)]
enum CoinType {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

struct Coin {
    value: CoinType,
}

impl Coin {
    fn value_in_cents(self) -> u8 {
        let value: u8 = match self.value {
            CoinType::Penny => 1,
            CoinType::Nickel => 5,
            CoinType::Dime => 10,
            CoinType::Quarter => 25,
        };

        value
    }
}

fn main() {
    let coin = Coin { value: CoinType::Dime };
    println!("{}", coin.value_in_cents());
}
