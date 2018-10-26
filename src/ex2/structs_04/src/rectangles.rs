#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
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
    println!("The area using of the rectangle using method is {} square pixels.", rect.area());
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

pub fn run() {
    println!("Running rectangles structs");
    rectangle_1();
}
