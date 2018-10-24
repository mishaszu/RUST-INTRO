mod methods;
mod cls;
mod hof;

fn print_value(x: i32) {
    println!("x: {}", x);
}

fn increase(x: &i32) -> i32 {
    *x + 1
}

fn functions() {
    print_value(20);
    let a: i32 = 2;
    print_value(increase(&a));
}

fn main() {
    functions();
    methods::run();
    cls::run();
    hof::run();
}
