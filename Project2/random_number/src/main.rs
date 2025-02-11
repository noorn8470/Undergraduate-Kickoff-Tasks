use std::io; // Import the `io` module from the standard library to handle user input.
use rand::Rng;// Import the `Rng` trait from the `rand` crate, enabling random number generation.

fn main() {  // Defines the main function, the entry point of the program.
    println!("Guess the number!");   // Print a welcome message to the console.
    // `let` declares an immutable variable `secret_number`.
    // `rand::thread_rng()` creates a random number generator instance.
    // `.gen_range(1..=100)` generates a random number between 1 and 100 (inclusive).
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");  // Uses `{secret_number}` to print the randomly generated number.
    
    println!("Please input your guess.");  // Prompt the user to input their guess.
    // `let` declares a variable.
    // `mut` makes the variable mutable (modifiable).
    // `String::new()` creates a new empty String instance.
    let mut guess = String::new();  // Create a mutable variable `guess` to store user input as a String.
    
    // Read user input from standard input (keyboard) and store it in `guess`.
    io::stdin() 
        // Calls `read_line`, passing a mutable reference to `guess`.
        .read_line(&mut guess)// Reads the user's input and appends it to `guess`
         // `expect` is used for error handling; if `read_line` fails, it will terminate the program and print the given error message.
        .expect("Failed to read line");// If reading fails, the program will crash with this message.

    // Uses `{}` as a placeholder for the `guess` variable in the formatted string.
    println!("You guessed: {}", guess);// Print the user's guess back to them.
}