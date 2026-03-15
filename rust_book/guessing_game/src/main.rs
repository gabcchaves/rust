use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please, input your guess.");

    let mut guess = String::new();

    io::stdin() // Returns a handle.
        .read_line(&mut guess)  // Method called on the returned handle.
        .expect("Failed to read line.");
    // The &mut indicates that the reference is mutable.

    println!("You guessed: {guess}.");
}
