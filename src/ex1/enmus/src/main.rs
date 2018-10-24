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
    // let a: Color = Color::RGBColor(255, 0, 1);
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

union IntOrFloat {
    i: i32,
    f: f32,
}

fn proccess_union(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => println!("Meaning of life"),
            IntOrFloat { f } => println!("f value: {}", f),
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 126 };
    unsafe { iof.i = 42 };
    let value = unsafe { iof.i };
    proccess_union(iof);
}

fn main() {
    enums();
    unions();
}
