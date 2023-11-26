fn main() {
    let x1: i32 = -98_222;
    println!("x1={}", x1);
    let x2: u8 = 0xff;
    println!("x2={}", x2);
    let x3: u16 = 0o77;
    println!("x3={}", x3);
    let x4: u8 = 0b1111_0000;
    println!("x4={}", x4);
    let x5: isize = 98_222;
    println!("x5={}", x5);
    let x6: u8 = b'A';
    println!("x6={}", x6);
    let x7 = 123u8;
    println!("x7={}", x7);
    let x8 = 2.0;
    println!("x8={}", x8);
    let x9: f32 = 3.0;
    println!("x8={}", x9);

    let tup: (u8, i32, f64) = (b'A', 123456, 12345.6789);
    let (x, y, z) = tup;
    println!("(x, y, z)=({}, {}, {})", x, y, z);
    println!("tup={:?}", tup);
    let tup = (12345886, 5.6789, b'Z');
    println!("tup={:?}", tup);
    println!("tup.1={}", tup.1);
    let tup = ();
    println!("tup={:?}", tup);
}
