fn strings_1() {
    let s1 = "initial string";
    let s2 = s1.to_string();
    let mut s3 = "initial string".to_string();

    s3.push_str(" from hello");
    println!("s3: {}", s3);

    let s4 = "test";
    s3.push_str(s4);
    println!("can I use s4 now?: {}", s4); //yes I can

    let s5 = s2 + &s3;
    println!("can I use s3 now?: {}", s3); //yes I can

    let mut s6 = String::from("hello");
    let s7 = String::from("world");
    s6.push(' ');
    let s6 = s6 + &s7;
    println!("can I use s6 now?: {}", s6); //yes I can
    println!("can I use s7 now?: {}", s7); //yes I can

    let s8 = String::from("hello");
    let s9 = String::from("world");
    let s10 = format!("{} {}", s8, s9);
    println!(
        "s10 is '{}', while can access s8: '{}' and s9: '{}'",
        s10, s8, s9
    );
}

fn strings_2() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("s: {}", s);
    for i in hello.chars() {
        println!("i in hello: {}", i);
    }
}

pub fn run() {
    println!("Running strings");
    strings_1();
    strings_2();
}
