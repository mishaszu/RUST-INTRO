fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue;
        }
        println!("while loop x = {}", x)
    }
    let mut y = 1;
    loop {
        y *= 2;
        println!("loop loop y = {}", y);
        if y == 1 << 10 {
            break;
        }
    }
}

pub fn run() {
    println!("Running while Loop");
    while_and_loop();
}
