use std::io;

fn fib(x: u64) -> u64 {
    if x < 2 {
        x
    } else {
        fib(x - 1) + fib(x - 2)
    }
}

fn fib2(x: u64) -> u64 {
    let mut memo: Vec<u64> = Vec::new();
    memo.push(0);
    if x > 0 {
        memo.push(1);
    }
    let mut i = 2;
    while i <= x {
        memo.push(memo.iter().rev().take(2).sum());
        i += 1;
    }
    *memo.last().unwrap()
}

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x: u64 = x.trim().parse().unwrap();
    if x <= 40 {
        println!("naive -> {}", fib(x));
    } else {
        println!("naive -> too large");
    }
    println!("memoized -> {}", fib2(x));
}
