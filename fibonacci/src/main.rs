use std::io;

fn fib(x: u64) -> u64 {
    if x < 2 {
        x
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x: u64 = x.trim().parse().unwrap();
    println!("{}", fib(x));
}
