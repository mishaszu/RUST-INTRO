struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

fn structures() {
    let p = Point { x: 2.0, y: 3.0 };
    let p2 = Point { x: 4.0, y: 5.0 };
    let line = Line { start: p, end: p2 };
    println!("start: ({}, {}), end: ({}, {})", line.start.x, line.start.y, line.end.x, line.end.y);
}

fn match_test() {
    let country_code = 1000;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        1...999 => "unknown",
        _ => "invalid",
    };

    println!("the country for this code is: {}", country);
}

fn main() {
    match_test();
    structures();
}
