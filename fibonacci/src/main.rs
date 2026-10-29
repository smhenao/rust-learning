//use std::io::{self, Write};

fn main() {
    println!("Welcome to the Fibonacci nth number generator.");
    //print!("Pick nth: ");
    //io::stdout().flush().unwrap();

    //let mut nth = String::new();
    //io::stdin().read_line(&mut nth).unwrap();

    //let mut nth: u32 = nth.trim().parse().expect("Please input a valid positive number");

    let mut nth: u32 = rand::random_range(1..=50);

    println!("Printing f{nth}");

    let mut prev_fib = 0;
    let mut fib = 1;

    // Temporary variable
    let mut fib_tmp: u64;

    loop {
        if nth == 1 {
            break;
        }
        fib_tmp = prev_fib;
        prev_fib = fib;
        fib = fib_tmp + fib;

        nth = nth - 1;
    }

    println!("Result: {fib}");
}
