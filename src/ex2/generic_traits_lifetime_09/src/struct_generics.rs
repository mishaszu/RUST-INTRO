struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn struct_g() {
    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2.2, y: 5.6 };
    let p3 = Point2 { x: 2, y: 4.0 };
    println!("p1 x: {}", p1.x());
}

pub fn run() {
    println!("Runnign struct generics");
    struct_g();
}
