fn how_many(x: u8) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        9...11 => "lof of",
        12 => "dozens",
        _ if (x % 2 == 0) => "some",
        _ => "few",
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }
}

pub fn run() {
    println!("Running pattern maching");
    pattern_matching();
}
