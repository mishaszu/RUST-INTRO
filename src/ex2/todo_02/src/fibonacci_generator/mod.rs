use std::io;

fn generate(v: &mut Vec<u32>, s: u32) -> u32 {
    if v.len() == 0 {
        let mut computed = vec![1, 1];
        generate(&mut computed, s)
    } else if s >= v.len() as u32 {
        let computed = v[v.len() - 2] + v[v.len() - 1];
        v.push(computed);
        generate(v, s)
    } else {
        v[s as usize]
    }
}

pub fn run() {
    println!("Running fibbonacci generator");
    let mut init_vector = vec![1, 1];
    println!("enter n-th number");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Can't parse input");
    let user_input: u32 = user_input.trim().parse().expect("Insert number");
    let result = generate(&mut init_vector, user_input - 1);
    println!("Nth number is: {}", result);
}
