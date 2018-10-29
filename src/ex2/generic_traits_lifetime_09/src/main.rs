//! The Rust Programming Language
//! Chapter 10
//! Generic types, tratis, and lifetimes

extern crate my_lib;

mod fn_generics;
mod struct_generics;
mod traits;

fn main() {
    fn_generics::run();
    struct_generics::run();
    traits::run();
}
