//! exercises from chapter 3
//! 1. Convert temperatures between Fahrenheit and Celsius.
//! 2. Generate the nth Fibonacci number.
//! 3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

mod fibonacci_generator;
mod song_printer;
mod temperatur_converter;

fn main() {
    println!("Hello, world!");
    // temperatur_converter::run();
    // fibonacci_generator::run();
    song_printer::run();
}
