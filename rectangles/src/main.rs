#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
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
    println!("area of {:?} is {}", &rect, rect.area());
    let smaller = Rect {
        width: 2,
        height: 2,
    };
    let larger = Rect {
        width: 12,
        height: 13,
    };
    let longer = Rect {
        width: 20,
        height: 1,
    };
    println!("Can hold the smaller rect?: {}", rect.can_hold(&smaller));
    println!("Can hold the larger rect?: {}", rect.can_hold(&larger));
    println!("Can hold the longer rect?: {}", rect.can_hold(&longer));
}

fn calculate_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
