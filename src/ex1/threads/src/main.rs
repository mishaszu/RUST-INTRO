mod ownership;
mod borrow;
mod lifetime;
mod referenceCounted;
mod atomicReferenceCounted;
mod mutex;

fn main() {
    ownership::run();
    borrow::run();
    lifetime::run();
    referenceCounted::run();
    atomicReferenceCounted::run();
    mutex::run();
}
