//! The Rust Programming Language
//! Chapter 4

mod references;
mod scope;
mod slices;

fn main() {
    scope::run();
    references::run();
    slices::run();
}
