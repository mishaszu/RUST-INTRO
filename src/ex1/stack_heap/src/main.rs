mod stack_heap;

fn heap() {
    let x = Box::new(5);
    println!("value from box = {}", *x);
}

fn main() {
    heap();
    stack_heap::run();
}
