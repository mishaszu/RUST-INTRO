mod atomicReferenceCounted;
mod borrow;
mod lifetime;
mod mutex;
mod ownership;
mod referenceCounted;

fn main() {
    ownership::run();
    borrow::run();
    lifetime::run();
    referenceCounted::run();
    atomicReferenceCounted::run();
    mutex::run();
}
