use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number game!!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Try to guess the number (1-100):");

        let mut guess = String::new();

        let _ = io::stdin().read_line(&mut guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("Your guess: {}", guess);

        if guess < secret_number {
            println!("Too small");
        } else if guess > secret_number {
            println!("Too big");
        } else {
            println!("You win!! The number was {}.", secret_number);
            break;
        }
    }
}
