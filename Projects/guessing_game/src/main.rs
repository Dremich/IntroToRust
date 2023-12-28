use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Welcome to Guess That Number!");
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>  {
                println!("You win!");
                break;
            }
        }
    }
}