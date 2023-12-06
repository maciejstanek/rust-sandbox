fn main() {
    let nth = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    let lines = [
        "A partridge in a pear tree.",
        "Two turtle doves, and",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas", nth[i]);
        println!("My true love sent to me");
        for j in (0..(i + 1)).rev() {
            println!("{}", lines[j]);
        }
        println!("");
    }
}
