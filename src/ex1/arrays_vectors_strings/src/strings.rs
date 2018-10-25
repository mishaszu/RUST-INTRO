fn strings() {
    let s: &'static str = "Hello world!";
    for c in s.chars().rev() {
        println!("char: {}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    };

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        if a as char != 'z' {
            letters.push_str(",");
        };
        a += 1;
    }
    println!("letters = {:?}", letters);
    let mut b = "hello world".to_string();
    b.push_str("!");
    println!("b = {}", b);
    println!("extra b = {}", b + " yoo");
}

pub fn run() {
    println!("Running strings");
    strings();
}
