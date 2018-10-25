//! Variables
use std::mem;
const MAX_VALUE: u32 = 100_000;

fn variables() {
    let x = 5;
    // x = 6; //can't assign new value to immutable variable
    let mut x = 3; //can shadow variable
    x = 6; //can assign to mutable variable
    let mut b = false;
    {
        let x = 10; //can shadow in inner scope
        b = true; //can assign to upper scope mutable
        println!("x in scope: {}", x);
    }
    println!("x outside scope: {}", x);
    println!("b after assign: {}", b);
    println!("Max: {}", MAX_VALUE);
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Length of string after reasigment: {}", spaces);
}

/// scalars
/// integers, floating-point number, booleans, characters
fn scalars() {
    /// define type to parse
    let x: u32 = "42".parse().expect("Not a number!");
    let y: String = "42".parse().expect("Not a number!");
    println!("x after prase: {}, y after parse: {}", x, y);

    /// integers
    let u1: u8 = 1;
    let u2: u16 = 1;
    let u3: u32 = 1; //default
    let u4: u64 = 1;
    let u5: u128 = 1;
    println!(
        "Unsigned integers:\n \tu8:{} size is {},\n\tu16:{} size is {},\n\tu32:{} size is {},\n\tu64:{} size is {},\n\tu128:{} size is {}",
        u1, mem::size_of_val(&u1),
        u2, mem::size_of_val(&u2),
        u3, mem::size_of_val(&u3),
        u4, mem::size_of_val(&u4),
        u5, mem::size_of_val(&u5)
    );

    let i1: i8 = 1;
    let i2: i16 = 1;
    let i3: i32 = 1; //default
    let i4: i64 = 1;
    let i5: i128 = 1;
    println!(
        "Unsigned integers:\n \ti8:{} size is {},\n\ti16:{} size is {},\n\ti32:{} size is {},\n\ti64:{} size is {},\n\ti128:{} size is {}",
        i1, mem::size_of_val(&i1),
        i2, mem::size_of_val(&i2),
        i3, mem::size_of_val(&i3),
        i4, mem::size_of_val(&i4),
        i5, mem::size_of_val(&i5)
    );

    let arch1: usize = 1;
    let arch2: isize = 1;
    println!(
        "System size integers:\n\tsystem unsigned size: {},\n\t system signed size: {}",
        mem::size_of_val(&arch1),
        mem::size_of_val(&arch2)
    );

    /// floating-point
    let f1: f32 = 1.0;
    let f2: f64 = 1.0; //default
    println!(
        "Floating points:\n\tf32:{} size is {},\n\tf64:{} size is {}",
        f1,
        mem::size_of_val(&f1),
        f2,
        mem::size_of_val(&f2)
    );

    /// bool
    let b1: bool = true;
    let b2 = false;
    println!(
        "Bool:\n\tb1:{} size is {},\n\tb2:{} size is {}",
        b1,
        mem::size_of_val(&b1),
        b2,
        mem::size_of_val(&b2)
    );

    /// char
    let c: char = 'a';
    println!("Char:\n\tchar:{} size is {}", c, mem::size_of_val(&c));
}

/// computed types
/// tuples, arrays
fn computed_types() {
    /// tuples
    let t1: (u8,) = (1,);
    let t2: (u32, i64, char) = (1, 1, 'a');
    println!(
        "Single tuple: {:?} size is {}, 3 variables tuple {:?} is size {}",
        t1,
        mem::size_of_val(&t1),
        t2,
        mem::size_of_val(&t2)
    );
    let (t2a, t2b, t2c) = t2;

    /// arrays
    let a1 = [1, 2, 3, 4, 5];
    let a2: [u32; 5] = [10, 11, 12, 13, 14];
    println!(
        "Arrays:\n\tarray {:?} is size of {},\n\t array {:?} is size of {}",
        a1,
        mem::size_of_val(&a1),
        a2,
        mem::size_of_val(&a2)
    );
}

pub fn run() {
    println!("Running basic");
    variables();
    scalars();
    computed_types();
}
