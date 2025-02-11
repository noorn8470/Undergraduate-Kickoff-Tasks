use std::io; // Import the `io` module from the standard library to handle user input.

fn main() {
    println!("Guess the number!");   // Print a welcome message to the console.

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