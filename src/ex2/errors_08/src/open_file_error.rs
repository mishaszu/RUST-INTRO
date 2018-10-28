use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn open_file_1() {
    let f = File::open("hello.txt");
    // let f:u32 = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem {:?}", e),
        },
        Err(error) => panic!("There was a problem openning file: {:?}", error),
    };
}

fn open_file_2() {
    // let f = File::open("hello1.txt").unwrap(); // run panic! if there is no file
    // let f = File::open("hello1.txt").expect("Can't open hello1.txt"); // run panic with custom and error message
}

fn open_file_3() -> Result<String, io::Error> {
    let f = File::open("hello_2.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn open_file_4() -> Result<String, io::Error> {
    let mut f = File::open("hello_2.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn open_file_5() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello_2.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub fn run() {
    println!("Running open file error");
    open_file_1();
    open_file_2();
    println!("file content: {:?}", open_file_3());
    println!("file content: {:?}", open_file_4());
    println!("file content: {:?}", open_file_5());
}
