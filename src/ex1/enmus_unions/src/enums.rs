enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CMYKColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn enums() {
    let a: Color = Color::CMYKColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };
    match a {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RGBColor(0, 0, 0)
        | Color::CMYKColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("Black"),
        Color::RGBColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => (),
    }
}

pub fn run() {
    println!("Running enums");
    enums();
}
