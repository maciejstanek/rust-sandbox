fn main() {
    let number = 7;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number != 0 {
        println!("the number was not a zero");
    }

    let number = 17;
    println!("{} is...", number);
    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by either 4, 3 or 2");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("selected {}", number);

    let mut count = 0;
    let result = loop {
        count += 1;
        println!("loop {}", count);
        if count == 10 {
            break count * 2;
        }
    };
    println!("result {}", result);
}
