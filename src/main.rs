#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.height
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state is :: {:#?} ::", state);
            25
        }
    }
}

#[derive(Debug)]
enum IPAddr {
    V4(u32, u32, u32, u32),
    V6(String),
}

fn main() {
    let rect = Rectangle {
        width: 23,
        height: 100,
    };

    println!("The total Area1 is :: {:?} ::", (&rect).area());

    let rect2 = Rectangle {
        width: 22,
        height: 90,
    };

    println!("The total Area2 is :: {:?} ::", rect2.area());

    println!("can rect2 hold rect1 :: {} ::", rect2.can_hold(&rect));
    println!("can rect1 hold rect2 :: {} ::", rect.can_hold(&rect2));

    let a = IPAddr::V6("::1".to_string());

    dbg!(a);
    dbg!(rect);

    println!(
        "The coin matched value is :: {} ::",
        value_in_cents(Coin::Penny)
    );
    println!(
        "The coin matched value is :: {} ::",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
}
