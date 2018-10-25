mod overloads;
mod static_dispatch;
mod traits;

fn main() {
    println!("Traits lessosn");
    traits::run();
    overloads::run();
    static_dispatch::run();
}
