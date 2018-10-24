trait Animal {
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
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hello!", self.name);
    }
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} meow hello!", self.name);
    }
}

fn traits() {
    let john = Human { name: "John" };
    john.talk();
    let misty = Cat {name: "Misty"};
    misty.talk();
}

pub fn run() {
    println!("Lesson 1");
    traits();
}
