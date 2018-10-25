//! if expression

fn simple_check() {
    let number = 3;

    let result = if number < 5 {
        println!("smaller");
        number + 3
    } else if number == 5 {
        println!("the same");
        number
    } else {
        println!("bigger");
        number - 3
    };
    println!("result from if expresion: {}", result);
}

pub fn run() {
    println!("Running if expression");
    simple_check();
}
