use std::io;

pub fn run() {
    println!("Running temp converter");
    println!("Insert Fahrenheit temp");

    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Can't read input");

    let x: i32 = x.trim().parse().expect("Insert number");
    let result = (x - 32) * 5 / 9;
    println!("{} Fahrenheits is {} Celsius", x, result);
}
