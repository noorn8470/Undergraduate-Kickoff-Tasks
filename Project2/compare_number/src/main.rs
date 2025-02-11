use rand::Rng; // Import the `Rng` trait from the `rand` crate, enabling random number generation.
use std::cmp::Ordering; // Allows comparison between numbers.
use std::io; // Import the `io` module from the standard library to handle user input.

fn main() {  // Defines the main function, the entry point of the program.
    println!("Guess the number!"); // Print a welcome message to the console.

    // Generate a secret random number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number); // Print the secret number for debugging.

    loop {
        println!("Please input your guess.");  // Prompt the user to input their guess.

        let mut guess = String::new();  // Create a mutable variable `guess` to store user input as a String.

        // Read user input from standard input (keyboard) and store it in `guess`.
        io::stdin()
            .read_line(&mut guess) // Reads the user's input and appends it to `guess`
            .expect("Failed to read line"); // If reading fails, the program will crash with this message.

        // Convert input string into a number (trim whitespace and parse as u32).
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If parsing is successful, use the number.
            Err(_) => {
                println!("Please enter a valid number!"); // Handle invalid input.
                continue; // Restart the loop.
            }
        };

        println!("You guessed: {}", guess); // Display the user's guess.

        // Compare the user's guess with the secret number.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"), // If the guess is lower.
            Ordering::Greater => println!("Too big!"), // If the guess is higher.
            Ordering::Equal => {
                println!("You win!"); // If the guess is correct.
                break; // Exit the loop.
            }
        }
    }
}
