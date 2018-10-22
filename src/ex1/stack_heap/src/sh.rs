#[allow(dead_code)]

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    println!("text from stack and heap module");
    let p1 = origin();
}
