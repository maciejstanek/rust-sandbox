use std::io;
use std::process;

const CELSIUS_OFFSET: f64 = 273.15;
fn c2k(c: f64) -> f64 {
    c + CELSIUS_OFFSET
}
fn k2c(k: f64) -> f64 {
    k - CELSIUS_OFFSET
}
const FARENHEIT_OFFSET: f64 = -32.0;
const FARENHEIT_FACTOR: f64 = 5.0 / 9.0;
fn f2c(f: f64) -> f64 {
    (f + FARENHEIT_OFFSET) * FARENHEIT_FACTOR
}
fn c2f(c: f64) -> f64 {
    (c / FARENHEIT_FACTOR) - FARENHEIT_OFFSET
}

fn main() {
    println!("=======================");
    println!(" Temperature Converter ");
    println!("=======================");
    println!("");

    println!("From what unit you want to convert?");
    println!("(c) from degrees Celsius");
    println!("(f) from degrees Farenheit");
    println!("(k) from Kelvins");
    let mut input_unit = String::new();
    io::stdin()
        .read_line(&mut input_unit)
        .expect("Failed to read input unit!");
    let input_unit = input_unit
        .to_lowercase()
        .chars()
        .next()
        .expect("Failed to read input unit!");

    println!("What value you want to convert?");
    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read input value!");
    let input_value: f64 = match input_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Input value is invalid!");
            process::exit(1);
        }
    };

    let value: f64 = match input_unit {
        'c' => c2k(input_value),
        'f' => f2c(c2k(input_value)),
        'k' => input_value,
        _ => {
            println!("Input unit not supported!");
            process::exit(1);
        }
    };

    println!("To what unit you want to convert?");
    println!("(c) to degrees Celsius");
    println!("(f) to degrees Farenheit");
    println!("(k) to Kelvins");
    let mut output_unit = String::new();
    io::stdin()
        .read_line(&mut output_unit)
        .expect("Failed to read output unit!");
    let output_unit = output_unit
        .to_lowercase()
        .chars()
        .next()
        .expect("Failed to read output unit!");

    let output_value: f64 = match output_unit {
        'c' => k2c(value),
        'f' => c2f(k2c(value)),
        'k' => value,
        _ => {
            println!("Output unit not supported!");
            process::exit(1);
        }
    };

    println!("The temperature is {} in the target unit.", output_value);
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.001;

    #[test]
    fn test_k2c() {
        assert!((0.0 - c2k(k2c(0.0))) < EPSILON);
        assert!((100.0 - c2k(k2c(100.0))) < EPSILON);
        assert!((300.0 - k2c(c2k(300.0))) < EPSILON);
    }

    #[test]
    fn test_c2f() {
        assert!((0.0 - f2c(c2f(0.0))) < EPSILON);
        assert!((100.0 - f2c(c2f(100.0))) < EPSILON);
        assert!((100.0 - c2f(f2c(100.0))) < EPSILON);
    }
}
