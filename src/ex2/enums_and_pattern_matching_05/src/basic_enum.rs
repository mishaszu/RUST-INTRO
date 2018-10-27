enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something with message
    }
}

fn enums_1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    //both four and six are the same type
    route(four); //valid
    route(six); // valid

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home2 = IpAddr3::V4(127, 0, 0, 1);
    let loopback2 = IpAddr3::V6(String::from("::1"));
}

fn route(id_type: IpAddrKind) {
    //...
}

fn enum_2() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

pub fn run() {
    println!("Running basic enum");
}
