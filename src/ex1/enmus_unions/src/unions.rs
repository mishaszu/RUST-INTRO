union IntOrFloat {
    i: i32,
    f: f32,
}

fn proccess_union(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => println!("Meaning of life"),
            IntOrFloat { f } => println!("f value: {}", f),
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 126 };
    unsafe { iof.i = 42 };
    let value = unsafe { iof.i };
    proccess_union(iof);
}

pub fn run() {
    println!("Running unions");
    unions();
}
