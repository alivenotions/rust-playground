// importing io library from the standard library
// this is for doing input/output operations
use std::io;

// fn declares a function and main is the entrypoint
// to our program a la cpp.
fn main() {
    // println! is a macro and NOT a function
    println!("Guess the number!");

    println!("Please input your guess");

    // let is for creating an immutable variable.
    // To make the variable mutable, add the mut keyword
    // before the variable name.
    // String::new creates a new instance of the String type.
    // :: means that the following function is an associated
    // function, which is like a static function in other
    // languages, which means that it is implemented on a type
    // and not on its instance.
    let mut guess = String::new();

    // stdin() is an associated function on io.
    io::stdin()
        // Takes whatever is in the standard input and places
        // that onto a string. Passing in a reference to the
        // string that needs to be mutated for the input.
        // It does not make sense to me yet as to why are we
        // not putting the ampersand on the variable name itself.
        .read_line(&mut guess)
        // io returns a Result type which basically a sum type of
        // OK | Error. Error needs to be handled with the expect
        // method.
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
