fn main() {
    println!("Hello, world!");
    another_fun(3, 'a');
}

fn another_fun(x: i32, y: char) {
    println!("another func ({} {})", x, y);
}
