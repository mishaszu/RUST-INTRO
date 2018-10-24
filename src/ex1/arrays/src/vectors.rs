fn vectors() {
    println!("vectors");
    let mut a = Vec::new();
    a.push(1);
    a.push(1);
    a.push(1);
    let idx: usize = 0;

    a[idx] = 321;
    // a[3] = 44;
    a.push(44);

    println!("a = {:?}, and is {} at 0", a, a[0]);

    println!("a at position 6 = {:?}", a.get(6));

    if let Some(z) = a.get(2) {
        println!("if let a at 5 position = {}", z)
    } else {
        println!("nothing here");
    }

    for i in &a {
        println!("value: {}", i);
    }

    println!("vector a length: {}", a.len());
    let last_element = a.pop();
    println!("last elemenet: {:?}, vector: {:?}", last_element, a);
    println!("vector a length: {}", a.len());

    while let Some(x) = a.pop() {
        println!("x: {}", x);
    }
    println!("vector a length: {}", a.len());
}

pub fn run() {
    println!("Running vectors");
    vectors();
}