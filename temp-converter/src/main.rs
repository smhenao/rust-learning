// self for stdin, Write for flushing
use std::io::{self, Write};

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Rust can't add floats and integers, so make 32 into a float
    (celsius * 1.8) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}

fn main() {
    println!("Welcome to the temperature converter\n");

    println!("Input the number according to the conversion you want:
1. Celsius to Fahrenheit
2. Fahrenheit to Celsius");

    print!("Input: ");
    io::stdout().flush().unwrap();

    let mut option = String::new();
    let mut temp = String::new();

    io::stdin().read_line(&mut option).unwrap();
    // Don't forget trim!
    let option: u32 = option.trim().parse().expect("Please input a number");

    print!("Input your temperature: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut temp).unwrap();
    let temp: f64 = temp.trim().parse().expect("Please input a temperature");

    if option == 1 {
        let result = celsius_to_fahrenheit(temp);
        println!("Your temperature is {result}°F.");
    } else if option == 2 {
        let result = fahrenheit_to_celsius(temp);
        println!("Your temperature is {result}°C.");
    } else {
        println!("Invalid choice.");
    }
}
