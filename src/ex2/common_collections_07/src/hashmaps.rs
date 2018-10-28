use std::collections::HashMap;

fn hashmaps_1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //return Option
    println!("{} team score is: {:?}", team_name, score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn hashmaps_2() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores: {:?}", scores);
}

fn hashmaps_3() {
    let team_name = String::from("Blue");
    let mut scores = HashMap::new();
    scores.insert(team_name, 10);
    println!("scores: {:?}", scores);
    // println!("team name: {}", team_name); //can't do that as team name was moved to hash map
}

fn hashmaps_4() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("scores: {:?}", scores);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(100);

    println!("scores: {:?}", scores);
}

fn hashmaps_5() {
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("count word: {:?}", map);
}

pub fn run() {
    println!("Running hash maps");
    hashmaps_1();
    hashmaps_2();
    hashmaps_3();
    hashmaps_4();
    hashmaps_5();
}
