fn borrow() {
    let print_vector = |x: &Vec<i32>| {
        println!("{:?}", x);
    };
    let v = vec![1, 2, 3];
    print_vector(&v);
    println!("vec: {:?}", v);

    {
        let mut a = 40;
        {
            let b = &mut a;
            *b += 2;
        }
        println!(" a = {}", a);
    }

    let mut z = vec![3, 2, 1];

    for i in &z {
        println!("i = {}", i);
        // can't push cuz it's protected by iterations
        // z.push(5);
    }
}

pub fn run() {
    println!("Running borrow");
    borrow();
}
