struct Person {
    name: Rc<String>,
}

use std::rc::Rc;

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn rc_demo() {
    let name = Rc::new("John".to_string());
    println!(
        "Name {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name)
    );
    {
        let person = Person::new(name.clone());
        println!(
            "Name {}, name has {} strong pointers",
            name,
            Rc::strong_count(&name)
        );
        person.greet();
    }
    println!(
        "Name {}, name has {} strong pointers",
        name,
        Rc::strong_count(&name)
    );
}

pub fn run() {
    println!("Running Reference Counted Variables");
    rc_demo();
}
