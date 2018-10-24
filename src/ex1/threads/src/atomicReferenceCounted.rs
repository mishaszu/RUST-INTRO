use std::rc::Rc;
use std::thread;
use std::sync::Arc;

struct Person {
    name: Arc<String>,
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn atomic() {
    let name = Arc::new("Misha".to_string());
    let person = Person::new(name.clone());

    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}", name);

    t.join().unwrap();
}

pub fn run() {
    println!("Running Atomic Reference Counted Variables");
    atomic();
}
