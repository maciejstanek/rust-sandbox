use rand::Rng;
use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("DBG: secret={}", secret);
    loop {
        print!("Input your guess: ");
        io::stdout().flush().unwrap(); // unwrap discards any errors
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read!");
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
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
