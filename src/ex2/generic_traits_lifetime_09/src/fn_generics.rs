fn largest(list: &[i32]) -> i32{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// Doesn't compile
// fn largest_2 <T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

fn find_1() {
    let number_list = vec![24, 56, 532, 51, 200];

    let result = largest(&number_list);
    println!("The biggest is: {}", result);

    let number_list = vec![24, 56, 11, 4124, 123, 532, 51, 200];
    let result = largest(&number_list);
    println!("The biggest is: {}", result);
}

pub fn run() {
    println!("Running function generics");
    find_1();
}