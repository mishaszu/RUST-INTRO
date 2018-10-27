#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quater state is {:?}", state);
            25
        }
    }
}

fn options_1() {
    let c1 = Coin::Quarter(UsState::Alabama);
    let v1 = value_in_cents(&c1);
    println!("c1 value is: {}", v1);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn options_2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn options_3() {
    let some_u8_valie = 0u8;
    match some_u8_valie {
        1 => println!("one"),
        2 => println!("two"),
        _ => (),
    }
}

fn options_4() {
    let v1 = Some(0u8);
    if let Some(0u8) = v1 {
        println!("Three");
    } else {
        println!("something else");
    }
}

pub fn run() {
    println!("Running options");
    options_1();
    options_2();
    options_3();
    options_4();
}
