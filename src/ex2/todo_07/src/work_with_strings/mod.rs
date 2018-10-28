fn make_pig_latin (s: &str) -> String {
    let mut s_chars = s.chars();

    let first_char = match s_chars.next() {
        Some(x) => x,
        None => return String::new(),
    };

    match first_char {
        'a' | 'e' | 'i' | 'o' | 'u' =>  format!("{}-hay", s),
        _ => format!("{}-{}ay", s_chars.as_str(), first_char)
    }
}

pub fn run() {
    println!("Running strings");
    let s1 = "first";
    println!("pig latin: {}", make_pig_latin(s1.clone()));
}