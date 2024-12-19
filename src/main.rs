use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;

    println!("Guess a number!");

    loop {

        println!("Please type your guess: ");
    
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to guess!");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // .expect("Please enter a number!");
    
        // println!("You guessed: {guess}");
        // println!("The Secret number was: {secret_number}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too Small");
                count += 1;
            },
            Ordering::Greater => {
                println!("Too Big");
                count += 1;
            },
            Ordering::Equal => {
                println!("You win!");
                count += 1;
                println!("Counts required to WIN : {count}");
                break;
            }
        }
    }

}
