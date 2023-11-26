fn main() {
    let mut x = 5;
    println!("x={}", x);
    x = 6;
    println!("x={}", x);
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("THREE_HOURS_IN_SECONDS={}", THREE_HOURS_IN_SECONDS);

    let a = 5;
    let a = a + 1;
    println!("outer before a={}", a);
    {
        let a = a * 2;
        println!("inner a={}", a);
    }
    println!("outer after a={}", a);
}
