//initializes standard input/output
use std::io;
//sets up the main function
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    //creates a mutable string called guess
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}