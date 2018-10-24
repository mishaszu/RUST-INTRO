fn use_slice(slice: &mut [i32]) {
    slice[0] = 321;
}

fn slices() {
    let mut data = [1, 2, 3, 4, 5, 6];
    use_slice(&mut data[1..4]);
    println!("data array = {:?}", data);
}

pub fn run() {
    println!("Running slices");
    slices();
}