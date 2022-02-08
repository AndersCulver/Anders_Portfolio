use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Create secret number to guess, print it for testing purposes
    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("The secret number is: {}", secret_number);
    
    loop {
        // Gather user input, convert string to number for compare
        println!("Please input your guess.");

        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Show guess to user
        println!("You guessed: {}", guess);

        // Compare guess to secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
