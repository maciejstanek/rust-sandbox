fn main() {
    println!("Hello, world!");
    another_fun(3, 'a');
}

fn another_fun(x: i32, y: char) {
    println!("another func ({} {})", x, y);

    let z = {
        let x = 12;
        x + 2
    };
    println!("z={}", z);
    let a = plus_one(five());
    println!("a={}", a);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
