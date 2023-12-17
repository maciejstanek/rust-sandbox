fn main() {
    let rect = (2, 6);
    println!("area={}", calculate_rectangle_area(rect));
}

fn calculate_rectangle_area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
