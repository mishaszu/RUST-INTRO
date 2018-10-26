fn scopes_1() {
    // no access to s
    let s = String::from("hello"); // s declaration
                                   // can access s
} // life of s i over here

fn scopes_2() {
    let x = 5;
    let y = x;
    let z = x + 2;
    println!("x: {}, y: {}, z: {}", x, y, z);

    let s1 = String::from("hello");
    let s2 = s1;
    // let s3 = s1 + " world!"; // Can't do that because s1 is moved to variable s2
    // println!("s3: {}", s3);

    let s3 = String::from("hello");
    let s4 = s3.clone() + " world";
    println!("s3: {}, s4: {}", s3, s4);

    let s5 = String::from("Hello");
    take_ownership(s5);
    // println!("s5: {}", s5); // can't do that because s5 was moved to the function

    let s6 = give_ownership(); // s6 owning string value
    let s7 = String::from("hello");
    let s8 = take_and_give_ownership(s7); // function take owner ship from s7 and return it to s8
}

fn take_ownership(x: String) {
    println!("taking ownership of: {}", x);
}

fn give_ownership() -> String {
    String::from("hello")
}

fn take_and_give_ownership(x: String) -> String {
    x
}

pub fn run() {
    println!("Running scope");
    scopes_1();
    scopes_2();
}
