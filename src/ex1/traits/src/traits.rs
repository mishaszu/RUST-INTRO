trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannon talk", self.name());
    }
}

struct Human {
    name: &'static str,
}

struct Cat {
    name: &'static str,
}

impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello!", self.name);
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} meow hello!", self.name);
    }
}

fn traits() {
    // simple create
    let john = Human { name: "John" };
    john.talk();
    let misty = Cat { name: "Misty" };
    misty.talk();
    // create by static
    let bob = Human::create("Bob");
    bob.talk();
    let sonic = Cat::create("Sonic");
    sonic.talk();

    let ana: Human = Animal::create("Ana");
    ana.talk();
}

pub fn run() {
    println!("Lesson 1");
    traits();
}
