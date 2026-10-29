fn takes_away(s1: String) {
    println!("{}", s1);
}

fn borrows(s2: &String) {
    // Despite s2 being a reference to a String, it can still be used directly as a String
    // Note: you can't modify anything you're borrowing, unless it is a mutable reference
    println!("{}", s2);
}

fn clones(s3: String) {
    println!("{}", s3);
}

fn mutable_borrow(s4: &mut String) {
    s4.push_str("Borrows");
    println!("{}", s4);
}

fn main() {
    let s1 = String::from("Hello World");
    takes_away(s1);

    // s1 can't be used anymore
    //println!("{}", s1); <-- this would error.

    let s2 = String::from("Hello Rust");
    borrows(&s2);

    // s2 can still be used
    println!("{}", s2);

    // An alternative is cloning the String
    let s3 = String::from("Hello References");
    clones(s3.clone());

    // s3 can still be used
    println!("{}", s3);

    // References can be mutable
    // Note: the original type has to be mutable too
    // Note: there can only exist a single mutable reference to a variable at once
    let mut s4 = String::from("Hello ");
    mutable_borrow(&mut s4);

    // s4 is now "Hello Borrows"
    println!("{}", s4);
}
