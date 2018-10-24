use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn stack_and_heap() {
    println!("text from stack and heap module");
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("size of struct: {}", mem::size_of_val(&p1));
    println!("size of struct: {}", mem::size_of_val(&p2));

    let p3 = *p2;
    println!(
        "x: {}, y: {}, size of struct: {}",
        p3.x,
        p3.y,
        mem::size_of_val(&p3)
    );
}

pub fn run() {
    println!("Running stack and heap");
    stack_and_heap();
}