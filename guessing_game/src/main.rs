use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("DBG: secret={}", secret);
    print!("Input your guess: ");
    io::stdout().flush().unwrap(); // unwrap discards any errors
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read!");
    println!("You guessed {}", guess);
}
