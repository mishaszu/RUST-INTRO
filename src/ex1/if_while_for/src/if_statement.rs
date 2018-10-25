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

pub fn run() {
    println!("Running if statement");
    if_statement();
}
