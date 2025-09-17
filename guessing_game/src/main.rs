use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Please guess a number between 1 and 100");

    let secret_key = rand::rng().random_range(1..=100);

    loop {
        println!("Please input your guess here");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess = guess.trim();

        if guess.eq_ignore_ascii_case("quit") || guess.eq_ignore_ascii_case("exit") {
            println!("Goodbye 👋");
            break;
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number, try again");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_key) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!!");
                break;
            }
        }
    }
}
