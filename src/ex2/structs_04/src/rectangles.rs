#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn rectangle_1() {
    let width1 = 30;
    let height1 = 50;

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect: {:#?}", rect);
    // println!("The area of the rectangle is {} square pixels.", area(rect));
    println!(
        "The area using of the rectangle using method is {} square pixels.",
        rect.area()
    );
}

fn rectangle_2() {
    let r1 = Rectangle {
        width: 30,
        height: 40,
    };
    let r2 = Rectangle {
        width: 20,
        height: 30,
    };
    let r3 = Rectangle {
        width: 40,
        height: 5,
    };
    let r4 = Rectangle {
        width: 5,
        height: 100,
    };
    println!(
        "Can rectangle r2 fit into rectangle r1?: {}",
        r1.can_hold(&r2)
    );
    println!(
        "Can rectangle r3 fit into rectangle r1?: {}",
        r1.can_hold(&r3)
    );
    println!(
        "Can rectangle r4 fit into rectangle r1?: {}",
        r1.can_hold(&r4)
    );
}

fn rectangle_3() {
    let r1 = Rectangle::new_square(20);
    println!("r1: {:#?}", r1);
}

pub fn run() {
    println!("Running rectangles structs");
    rectangle_1();
    rectangle_2();
    rectangle_3();
}
