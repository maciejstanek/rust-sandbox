fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    let mut s1 = String::from("hello");
    change(&mut s1);
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let three_words = String::from("lorem ipsum dolor");
    println!(
        "first word index of '{}' is {}",
        three_words,
        first_word(&three_words)
    );
    let one_word = String::from("duck");
    println!(
        "first word index of '{}' is {}",
        one_word,
        first_word(&one_word)
    );
    println!("string slice 2..8 -> {}", &three_words[2..8]);
    println!("string slice 2.. -> {}", &three_words[2..]);
    println!("string slice ..8 -> {}", &three_words[..8]);
    println!("string slice .. -> {}", &three_words[..]);
    println!(
        "first word of '{}' is {}",
        three_words,
        first_word2(&three_words)
    );
    println!("first word of '{}' is {}", one_word, first_word2(&one_word));
}

fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
