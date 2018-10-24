#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

use std::ops::Add;

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

fn overload() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 12.0 };
    let p3 = p1 + p2;
    println!("p3: {:?}", p3)
}

pub fn run() {
    println!("Running overload");
    overload();
}
