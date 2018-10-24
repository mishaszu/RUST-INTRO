extern crate rand;
use rand::Rng;
extern crate customCrate;
use customCrate::greetings::{english, french};

fn main() {
    let mut rng = rand::thread_rng();
    let b: bool = rng.gen();
    println!("random boolean is: {}", b);
    println!("English: {}, {}", english::hello(), english::goodbye());
    println!("French: {}, {}", french::hello(), french::goodbye());
}
