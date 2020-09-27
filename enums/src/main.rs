#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
 
    println!("home: {:#?}", home);
    println!("loopback: {:#?}", loopback);
    
    let my_coin = Coin::Dime;
    let my_other_coin = Coin::Quarter(UsState::Washington);
    
    let value = value_in_cents(my_coin);
    let value = value_in_cents(my_other_coin);
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
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
            println!("State from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

