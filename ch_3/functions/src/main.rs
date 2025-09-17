fn main() {
    paramater_fucntion(5);
    print_labeled_measurement(32, 'h');
    express();

    let x = five();

    println!("The value of x is: {x}");
}

fn paramater_fucntion(x: i32) {
    println!("This is a number {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The labeled measurement is: {value}{unit_label}");
}

// expressions

fn express(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}
