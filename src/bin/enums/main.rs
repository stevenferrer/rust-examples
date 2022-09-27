use std::mem::Discriminant;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(c: Coin) -> u8 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let m = Message::Write(String::from("asdf"));
    m.call();

    let some_num = Some(3);
    let some_char = Some('a');

    let absent_num: Option<i32> = None;

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max is configured to be {}.", max);
    } else {
        println!("No max config.")
    }
}
