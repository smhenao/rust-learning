fn main() {
    println!("Welcome to the Fibonacci nth number generator.");

    let mut nth: u32 = rand::random_range(1..=50);

    println!("Printing f{nth}");

    let mut fib = (0, 1);

    // Temporary variable
    let mut fib_tmp: u64;

    loop {
        if nth == 1 {
            break;
        }

        fib_tmp = fib.0;
        fib.0 = fib.1;
        fib.1 = fib_tmp + fib.1;

        nth = nth - 1;
    }

    println!("Result: {}", fib.1);
}
