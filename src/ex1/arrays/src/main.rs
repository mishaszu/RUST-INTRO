use std::mem;

fn arrays() {
    println!("arrays");
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is: {}", a.len(), a[0]);

    a[0] = 321;
    println!("now first is: {}", a[0]);
    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5] {
        println!("does not match");
    }

    let b = [1u64; 10];
    for i in 0..b.len() {
        println!("{}: {}", i, b[i]);
    }
    for i in &b {
        println!("value: {}", i);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));

    let mtx: [[f32; 3]; 2] = [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0]];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

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

fn use_slice(slice: &mut [i32]) {
    slice[0] = 321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5, 6];
    use_slice(&mut data[1..4]);
    println!("data array = {:?}", data);
}

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

fn main() {
    arrays();
    vectors();
    slices();
    strings();
}
