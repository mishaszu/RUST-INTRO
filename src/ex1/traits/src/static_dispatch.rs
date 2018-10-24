trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

struct Circle {
    radius: f64,
}
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

//dynamic dispatch
fn print_it_too(z: &Printable) {
    println!("{}", z.format());
}

//static dispatch
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

//dynamic dispatch
fn dynamic_d() {
    println!("dynamic dispatch");
    let shapes: [&Shape; 4] = [
        &Circle { radius: 1.0 },
        &Square { side: 3.0 },
        &Circle { radius: 2.0 },
        &Square { side: 4.0 },
    ];
    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area: {}", i, shape.area());
    }
}

pub fn run() {
    println!("Running static dispatch");
    let a = 123;
    let b = "hello".to_string();
    {
        print_it(a);
        print_it(b);
    }
    let c = 456;
    let d = "world".to_string();
    print_it_too(&c);
    print_it_too(&d);
    dynamic_d();
}
