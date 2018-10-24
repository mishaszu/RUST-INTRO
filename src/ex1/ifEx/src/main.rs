fn if_statement() {
    let temp = 26;

    if temp > 30 {
        println!("really hot");
    } else if temp < 10 {
        println!("really cold");
    } else {
        println!("temparature is ok");
    }

    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("day is {}", day);
    println!(
        "it is {}",
        if temp > 20 {
            if temp > 22 {
                "very hot"
            } else {
                "only hot"
            }
        } else {
            "cold"
        }
    );
}

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

fn main() {
    // if_statement();
    // while_and_loop();
    for_loop();
}
