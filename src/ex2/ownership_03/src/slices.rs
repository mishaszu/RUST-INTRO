fn frist_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn frist_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn frist_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}

fn slice_1() {
    let mut s = String::from("Hello");
    let word = frist_word(&s);
    s.clear();
    println!("word: {}", word);

    let s2 = String::from("Hello world!");
    let slice1 = &s2[..5];
    let slice2 = &s2[5..];
    println!("part1: '{}' and part2: '{}'", slice1, slice2);

    let s3 = String::from("Hello world!");
    println!("Slice: {}", frist_word3(&s3[..]));
}

pub fn run() {
    println!("Running slices");
    slice_1();
}
