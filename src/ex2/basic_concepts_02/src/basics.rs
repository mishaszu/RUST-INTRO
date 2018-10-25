//! Variables

fn variables() {
    let x = 5;
    // x = 6; //can't assign new value to immutable variable
    let mut x = 3; //can shadow variable
    x = 6; //can assign to mutable variable
    let mut b = false;
    {
        let x = 10; //can shadow in inner scope
        b = true; //can assign to upper scope mutable
        println!("x in scope: {}", x);
    }
    println!("x outside scope: {}", x);
    println!("b after assign: {}", b);
}

pub fn run() {
    println!("Running basic");
    variables();
}
