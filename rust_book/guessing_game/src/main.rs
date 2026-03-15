use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please, input your guess.");

    let mut guess = String::new();

    io::stdin() // Returns a handle.
        .read_line(&mut guess)  // Method called on the returned handle.
        .expect("Failed to read line.");
    // The &mut indicates that the reference is mutable.

    println!("You guessed: {guess}.");

    println!("The secret number is {secret_number}.");
}
