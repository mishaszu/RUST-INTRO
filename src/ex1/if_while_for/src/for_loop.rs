fn for_loop() {
    for x in 1..11 {
        if x == 3 {
            continue;
        }
        if x == 7 {
            break;
        }
        println!("for loop x = {}", x);
    }

    for (pos, y) in (31..40).enumerate() {
        println!("{}: {}", pos, y);
    }
}

pub fn run() {
    println!("Running for loop");
    for_loop();
}