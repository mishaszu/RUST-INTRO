fn calculate_length(x: &String) -> usize {
    x.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn change_and_return(s: String) -> String {
    s + " world!"
}

fn borrow_with_reference() {
    let s1 = String::from("hello");
    let s1_length = calculate_length(&s1);
    println!("s1: '{}' length is: {}", s1, s1_length);
}

fn mutable_reference() {
    let mut s1 = String::from("Hello");
    change(&mut s1);
    println!("s1 is {}", s1);
    // can't execute this way
    // let s2 = String::from("Hello");
    // let s3 = change_and_return(&s2);
    // println!("s2 is: '{}' while s3 is {}", s2, s3);
    // to complete that I need copy
    let s2 = String::from("Hello");
    let s2 = change_and_return(s2);
    println!("s2: {}", s2);
}

pub fn run() {
    println!("Running references");
    borrow_with_reference();
    mutable_reference();
}
