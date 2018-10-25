//! The Rust Programming Language
//! Chapter 2

extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn random_gen() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn main() {
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

        println!("You guessed: {}", guess);
        match guess.cmp(&r1) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
