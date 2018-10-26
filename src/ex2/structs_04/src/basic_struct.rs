struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn struct_1() {
    let user1 = User {
        email: String::from("my_email@mail.com"),
        active: false,
        sign_in_count: 100,
        username: String::from("user"),
    };
    let user2 = build_user(String::from("Mike"), String::from("mike@email.com"));
    // println!("user2: {}", user2);
    let user3 = User {
        email: String::from("some_email@email.com"),
        username: String::from("some name"),
        ..user2
    };
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

pub fn run() {
    println!("Running ");
    struct_1();
    tuple_struct();
}
