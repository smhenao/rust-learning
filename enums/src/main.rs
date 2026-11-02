// Each variant of an enumeration can have different associated values
enum Color {
    RGB{ r: u8, g: u8, b: u8 },
    Hex(u32),
    Nothing,
}

// Just as structs can have methods, so can enums
impl Color {
    fn print(&self) {
        // To access the associated values of an enum, first we have to
        // check what type of enum is it
        match self {
            // The name of the variables we put inside the associated values of the enum
            // are the values we can use inside the match
            Color::RGB{r, g, b} => println!("{}, {}, {}", r, g, b),
            Color::Hex(hex) => println!("{:x}", hex),
            Color::Nothing => println!("Not a color"),
        }
    }
}

fn main() {
    // Despite all of them having a different amount and different types of
    // associated values, they're still of type "Color"
    let rgb_color = Color::RGB{r: 255, g: 255, b: 255};
    let hex_color = Color::Hex(0x505050);
    let no_color = Color::Nothing;

    rgb_color.print();
    hex_color.print();
    no_color.print();

    // A commonly used enum in Rust is Option
    // Option is a generic type enum
    let something = Some(123);
    let nothing: Option<i32> = None;

    // Once again, the compiler forces us to handle both cases of the enum
    // to make sure we don't access a value that doesn't exist
    match something {
        Some(value) => println!("{}", value),
        None => println!("It's empty"),
    }

    match nothing {
        Some(value) => println!("{}", value),
        None => println!("It's empty"),
    }
}
