mod sh;

fn heap() {
    let x = Box::new(5);
    println!("value from box = {}", *x);
}

fn main() {
    heap();
    sh::stack_and_heap();
}