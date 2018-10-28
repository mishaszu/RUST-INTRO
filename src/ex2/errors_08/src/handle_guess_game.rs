//! The Rust Programming Language
//! Chapter 2
//!

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, go {}", value);
        }
        Guess { value }
    }
    pub fn value(self) -> u32 {
        self.value
    }
}

fn random_gen() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

pub fn run() {
    println!("Guessing game!");
    let r1 = random_gen();

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        let checker = Guess::new(guess);

        println!("You guessed: {}", guess);
        match checker.value().cmp(&r1) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
