struct Person {
    name: String,
}

impl Person {
    fn get_ref_name(&self) -> &String {
        &self.name
    }
    fn get_ref_name2<'a>(&'a self) -> &'a String {
        &self.name
    }
}

struct Company<'z> {
    name: String,
    ceo: &'z Person,
}

fn lifetime() {
    let boss = Person {
        name: String::from("Misha Szu"),
    };
    let brand = Company {
        name: String::from("My company"),
        ceo: &boss,
    };
    let mut z: &String;
    {
        let p = Person {
            name: String::from("someName"),
        };
        z = p.get_ref_name();
        z = p.get_ref_name2();
    }
}

pub fn run() {
    println!("Running lifetime");
    lifetime();
}
