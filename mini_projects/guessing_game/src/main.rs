// A guessing game

// Load module into scope
use std::io;
use std::cmp::Ordering; // enum for comparing two things. can be Less, Greater, Equal
use rand::Rng; // range
use colored::*; // for colored terminal output


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0, 101); // 0-100 inclusive

    // println!("The secret number is: {}", secret_number); // debug
    let mut attempts: u32 = 0;

    loop {
        println!("Please input your guess.");

        // mut : mutable (per default immutable)
        // String class
        // ::new() is like a static method (associated function)
        //      returns empty string for us to use.
        let mut guess = String::new();

        io::stdin() // handle to std-input
            .read_line(&mut guess) // & = reference (like a pointer). Returns std::result::Result enum can either be Ok or Err.
            .expect("Failed to read line"); // if Err, crash and display message.

        // shadowing: we can reuse the same variable name
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue; // skip to next iteration of loop
            },
        };
        attempts += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { // cmp = compare
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                // println!("{} {}", "You win".green().bold(), format!("in {} attempts!", attempts).green().bold());
                println!("{}", format!("You win in {} attempts!", attempts).green().bold());
                break; // exit loop
            },
        } 
    }
    
}
