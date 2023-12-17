#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 5;
    let rect = Rect {
        width: dbg!(2 * scale),
        height: 6,
    };
    println!("{:#?}", rect);
    dbg!(&rect);
    println!("area of {:?} is {}", &rect, calculate_area(&rect));
}

fn calculate_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
