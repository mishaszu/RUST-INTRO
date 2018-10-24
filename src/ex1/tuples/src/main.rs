fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("sum = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);
    let sp2 = sum_and_product(5, 8);
    let combo = (sp, sp2);
    println!("combo = {:?}", combo);
    println!("combo value: {}", (combo.1).0);
    let ((c, d), (e, f)) = combo;
    println!("c: {}, d: {}, e: {}, f: {}", c, d, e, f);
}

fn main() {
    tuples();
}
