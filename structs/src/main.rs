#[derive(Debug)]
struct Rectangle {
    height: u64,
    width: u64,
}

fn calculate_area(rect: &Rectangle) -> u64 {
    // No semicolon needed because it's our implicit return value
    rect.height * rect.width
}

#[derive(Debug)]
struct Triangle {
    height: f64,
    width: f64,
}

// We can also implement methods inside structures
// Everything inside this impl will be associated with the Triangle struct/type
// NOTE: impl blocks can be separated just fine
impl Triangle {
    // Here &self, the first argument, equals to
    // self: &Type
    // and Type is an alias for the type of the impl
    fn area(&self) -> f64 {
        (self.width * self.height) / 2.0
    }
    // NOTE: There can be methods with the same name as the struct fields
}

impl Rectangle {
    // Structures can also have associated functions
    // Associated functions don't use self
    // And are used with the :: syntax
    fn square(size: u64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        height: 4,
        width: 4,
    };

    println!("The area is: {}", calculate_area(&rect));

    // Trying to print the struct normally would fail...
    // println!("Structure: {rect}");

    // However Rust print format allows us to add :? to use Debug output
    // NOTE: requires the struct to #[derive(Debug)]
    println!("Structure: {rect:?}");

    // dbg! is also a very useful macro for debugging
    // it prints the result of an expression to stderr
    let x = 2;
    let rect2 = Rectangle {
        // dbg! takes ownership then returns it
        height: dbg!(x * 30),
        width: 30,
    };

    dbg!(&rect2);

    let tri = Triangle {
        height: 4.0,
        width: 4.0,
    };

    println!("Triangle area: {}", tri.area());

    // Using an associated function
    let square = Rectangle::square(4);
    println!("Area of the square: {}", calculate_area(&square));
}
