use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Lets play a guessing game!!!");

    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is {secret_number}");

    loop {
        println!("Please enter your guess.");

        //declaring a mutable vairable lo store the guess
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Please input intergers only");
                continue;
            }
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
