use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number between 1 and 100!");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        print!("Input your guess: ");
        io::stdout().flush().unwrap(); // unwrap discards any errors
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
