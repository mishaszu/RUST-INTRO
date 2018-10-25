const MEANING_OF_LIFE: u8 = 42;
static z: i32 = 123;

fn scope1() {
    let a = 123;
    {
        let b = 456;
        println!("b in new scope = {}, a in nested scope = {}", b, a);

        let a = 321;
        println!("inner a = {}", a);
    }
    println!("outer a = {}", a);
}

fn main() {
    scope1();
    println!("{}", MEANING_OF_LIFE);
    println!("{}", z)
}
