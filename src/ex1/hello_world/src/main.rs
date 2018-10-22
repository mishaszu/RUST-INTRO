use std::mem;

fn main() {
    println!("Hello, world!");
    let a: u8 = 123;
    println!("a = {}", a);

    let mut b:i8 = 0;
    println!("b = {}", b);
    b = 125;
    println!("b = {}", b);

    let mut c = 12456789;
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} after mod, size = {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, take up {}, {}-bit os", z, size_of_z, size_of_z * 8);

    let d = 'c';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e:f32 = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}
