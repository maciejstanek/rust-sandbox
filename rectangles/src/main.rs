#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect {
        width: 2,
        height: 6,
    };
    println!("area of {:?} is {}", &rect, calculate_area(&rect));
}

fn calculate_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
