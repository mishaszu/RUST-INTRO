//! loops

fn loop_loop() {
    let mut x = 0;
    loop {
        x += 1;
        if x ==3 {
            continue;
        }
        println!("looping loop with x: {}", x);
        if x == 5 {
            break;
        }
    }
}

fn while_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut idx = 0;
    while idx < 5 {
        println!("the value is: {}", a[idx]);
        idx += 1;
    }
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("number is {}", number);
    }
}

pub fn run() {
    println!("Running loops");
    loop_loop();
    while_loop();
    for_loop();
}
