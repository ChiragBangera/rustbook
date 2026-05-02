fn main() {
    let mut x: i32 = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y: i32 = 5;

    let y: i32 = y + 1;

    println!("The value of y is: {y}");

    //creting a tuple.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("This is a tuple {tup:?}");

    let five_hundred: i32 = tup.0;

    let six_point_four: f64 = tup.1;

    let one: u8 = tup.2;

    println!(
        "These are the deconstructed values from a tuple: {five_hundred}, {six_point_four} and last index {one}"
    );

    let a = [3; 5];
    println!("{a:?}");

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{b:?}")
}
