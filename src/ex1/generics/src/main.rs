struct Point<T> {
    x: T,
    y: T,
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>,
}

fn generics() {
    let a: Point<u32> = Point { x: 2, y: 1 };
    let b: Point<f64> = Point { x: 2.2, y: 5.1 };
    let c: Line<u32> = Line {
        start: Point { x: 1, y: 1 },
        end: Point { x: 0, y: 20 },
    };
}

fn main() {
    generics();
}
