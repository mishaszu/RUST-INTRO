fn hello() {
    println!("Hello world!");
}

fn closures() {
    let a = hello;
    a();
    let b = |x: u32| -> u32 { x + 1 };
    let c = 2;
    println!("increse c({}) by one: {}", c, b(c));
    let mut two = 2;
    {
        let add_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("addTwo: {}", add_two(2));
    }

    let borrow_two = &mut two;
}

pub fn run() {
    println!("Running closures");
    closures();
}
