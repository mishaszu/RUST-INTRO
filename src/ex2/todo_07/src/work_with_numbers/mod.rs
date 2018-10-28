use std::collections::HashMap;

//mean
fn mean(v: &Vec<i32>) -> f64 {
    let mut sum: i32 = 0;
    for i in v {
        sum += *i;
    }
    sum as f64 / v.len() as f64
}

//median

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let index: usize = {
        if (v.len() % 2 == 0) {
            (v.len() / 2) as usize - 1
        } else {
            (v.len() as f64 / 2.0).floor() as usize
        }
    };
    v[index]
}

//mode
fn mode(v: &Vec<i32>) -> HashMap<i32, i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in v {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    };
    map
}

pub fn run() {
    println!("Running work with numbers");
    let mut v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut v2: Vec<i32> = vec![124, 5151, 52341, 3213, 543, 124, 543];
    println!("mean is {}", mean(&v1));
    println!("median is {}", median(&mut v1));
    println!("median is {}", median(&mut v2));
    println!("can I use v?: {:?}", v1); //yes because I moved it to function that is executed, but it's sorted
    println!("mode is: {:?}", mode(&v2));
}
