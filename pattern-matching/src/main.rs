enum Texts {
    Spanish(String),
    English(String),
}

impl Texts {
    fn print_if_english(&self) {
        // Instead of using match for one pattern and ignoring the rest
        // match self {
        //     Texts::English(text) => println!("{}", text),
        //     _ => println!("Text isn't English!"),
        // }
        // We can just use if let
        if let Texts::English(text) = self {
            println!("{}", text);
        } else {
            println!("Text isn't English!");
        }
    }
}

fn main() {
    let hello_world = Texts::English(String::from("Hello, world!"));
    let hola_mundo = Texts::Spanish(String::from("Hola mundo!"));

    hello_world.print_if_english();
    hola_mundo.print_if_english();
}
