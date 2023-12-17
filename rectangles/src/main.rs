fn main() {
    let width = 2;
    let height = 6;
    println!("area={}", calculate_rectangle_area(width, height));
}

fn calculate_rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}
