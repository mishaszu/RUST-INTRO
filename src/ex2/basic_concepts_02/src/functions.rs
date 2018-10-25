//! functions

fn parameters(x: u32, y: u64) -> u32 {
    x + y as u32
}

fn expression_and_statement() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}, and value of x is: {}", y, x);
}

fn before() {
    println!("you can declare before");
}

pub fn run() {
    println!("Running functions");
    before();
    after();
    let a = parameters(20, 30);
    println!("Function result is: {}", a);
    expression_and_statement();
}


fn after() {
    println!("you can declare after");
}
