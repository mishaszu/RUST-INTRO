fn owner() {
    let v1 = vec![1, 2, 3];
    // let v2 = v1;

    // can't print as v1 was moved
    // println!("{:?}", v1);

    let u = Box::new(1);
    let u2 = u;
    // can't print u as u was moved
    // println!("u = {}", *u);

    let print_vector = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };

    let vv = print_vector(v1);
    println!("{}", vv[0]);
}

pub fn run() {
    println!("Running ownership");
    owner();
}
