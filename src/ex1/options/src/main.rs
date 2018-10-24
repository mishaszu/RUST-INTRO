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
}

fn main() {
    options();
}
