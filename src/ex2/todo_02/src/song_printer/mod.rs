mod elements;
mod nths;

fn intro_formater(n: usize) -> String {
    "On the ".to_string() + nths::get_nth(n) + " day of Christmas, my true love gave to me\n"
}

fn printer() {
    for i in 0..12 {
        let intro = intro_formater(i);
        let elements = elements::append_element(i);
        let say = intro + &elements.to_owned() + "\n";
        println!("{}", say);
    }
}

pub fn run() {
    println!("Running song printer");
    printer();
}
