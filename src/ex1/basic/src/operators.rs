fn operators() {
    let a = 2 + 2 * 3;
    println!("a = {}", a);
    println!("a % = {}", a % 3);
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    let mut c = 1 | 2;
    println!("c = {}", c);
    c = 1 << 10;
    println!("c = {}", c);

    let pi_less_4 = std::f64::consts::PI < 4.0;
    let higher = 2 >= 2;
    println!("pi_less_4 = {}, higher = {}", pi_less_4, higher);
}

pub fn run() {
    println!("Running operators");
    operators();
}
