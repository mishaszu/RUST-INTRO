struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub fn run() {
    println!("Running methods");
    let p1 = Point { x: 1.0, y: 3.0 };
    let p2 = Point { x: 4.0, y: 5.0 };
    let l1 = Line { start: p1, end: p2 };
    println!("l1 length = {}", l1.len());
}
