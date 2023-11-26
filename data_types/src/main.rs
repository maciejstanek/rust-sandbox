use std::io;

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

    let ar = [1, 45, 33, -4];
    println!("ar={:?}", ar);
    let ar: [i32; 5] = [1, 4, 7, 2, 1];
    println!("ar={:?}", ar);
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months={:?}", months);
    let may = months[4];
    println!("may={:?}", may);
    let ar = [3; 5];
    println!("ar={:?}", ar);

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
