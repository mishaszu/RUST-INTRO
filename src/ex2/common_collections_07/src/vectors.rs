fn vectors_1() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3: Vec<i32> = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    let subV1: &i32 = &v2[1];
    let subV2: Option<&i32> = v2.get(1);

    for i in &v2 {
        println!("i: {}", i);
    }

    for i in &mut v3 {
        *i *= 50;
    }

    println!("v3: {:?}", v3);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors_2() {
    let v1 = vec![
        SpreadsheetCell::Int(22),
        SpreadsheetCell::Float(0.0),
        SpreadsheetCell::Text(String::from("hello")),
    ];
}

pub fn run() {
    println!("Running vectors");
    vectors_1();
    vectors_2();
}
