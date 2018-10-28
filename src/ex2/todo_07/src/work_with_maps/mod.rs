use std::collections::HashMap;
use std::iter::FromIterator;
use std::iter::Iterator;

fn get_in_department(dep: &str, map: &HashMap<String, String>) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    for (name, department) in map {
        if department == dep {
            v.push(name.clone());
        }
    }
    v
}

fn add(input: &str, map: &mut HashMap<String, String>) {
    let words = input.split_whitespace();
    let mut name: Option<&str> = None;
    let mut department: Option<&str> = None;
    for (index, key) in words.enumerate() {
        if index == 1 {
            name = Some(key);
        } else if index == 3 {
            department = Some(key);
        }
    }
    match (name, department) {
        (Some(x), Some(y)) => {
            map.entry(x.to_string()).or_insert(y.to_string());
        }
        _ => (),
    }
}

pub fn run() {
    println!("Running Maps");
    let mut map: HashMap<String, String> = HashMap::new();
    println!("map is: {:?}", map);
    add("Add Sally to Engineering", &mut map);
    add("Add Eve to Engineering", &mut map);
    println!("map is: {:?}", map);
    add("Add Mark to Sales", &mut map);
    println!("map is: {:?}", map);
    println!(
        "In Engineering department work: {:?}",
        get_in_department("Engineering", &map)
    );
    let test = "test atest";

    let mut chars: Vec<char> = test.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    let s = String::from_iter(chars);
    println!("{}", s);
}
