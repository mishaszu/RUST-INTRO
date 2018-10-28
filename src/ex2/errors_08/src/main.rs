//! The Rust Programming Language
//! Chapter 9
//! Error Handling

mod open_file_error;
mod handle_guess_game;

fn main() {
    // panic!("Crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];
    open_file_error::run();
    handle_guess_game::run();
}
