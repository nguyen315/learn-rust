use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess");
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess is: {}", guess);
    
        println!("Compare your guess...");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Small!"),
            Ordering::Greater => println!("Big!"),
            Ordering::Equal => {
                println!("That's correct !!!");
                break;
            }
        }
    }
}