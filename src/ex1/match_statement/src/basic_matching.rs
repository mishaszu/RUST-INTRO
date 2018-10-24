fn match_test() {
    let country_code = 1000;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        1...999 => "unknown",
        _ => "invalid",
    };

    println!("the country for this code is: {}", country);
}

pub fn run() {
    println!("Running basic maching");
    match_test();
}
