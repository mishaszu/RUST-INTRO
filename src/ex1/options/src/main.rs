fn options() {
    let x = 3.0;
    let y = 2.0;

    let result: Option<f64> = if y != 0.0 { Some(x / y) } else { None };

    println!("{:?}", result);

    match result {
        Some(x) => println!("result: {}", x),
        None => println!("Can't divide {}/{}", x, y),
    }

    if let Some(z) = result {
        println!("if let result: {}", z);
    };

    let x: [u8; 3] = [1, 2, 3];

    if let Some(z) = x.get(3) {
        println!("z: {}", z);
    } else {
        println!("no match");
    }
}

fn main() {
    options();
}
