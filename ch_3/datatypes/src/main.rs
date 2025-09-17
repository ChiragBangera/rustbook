use std::io;

fn main() {
    tuples();
    invalid_array_element();
}

fn tuples() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;

    println!("The value of y is: {y}");
    println!("The value of x is: {x}");
    println!("The value of z is: {z}");
    println!("Five hundred: {five_hundred}");
}

fn invalid_array_element() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter the index you want to access:");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("index expected was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
