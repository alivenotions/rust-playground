// importing io library from the standard library
// this is for doing input/output operations
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// fn declares a function and main is the entrypoint
// to our program a la cpp.
fn main() {
    // println! is a macro and NOT a function
    println!("Guess the number!");

    // gen_range: [x, y)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
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
            // I am confused as to why we are putting the
            // ampersand on the `mut` keyword and not on the
            // variable name.
            .read_line(&mut guess)
            // io returns a Result type which is basically a sum type of
            // OK | Error. Error needs to be handled with the expect
            // method.
            .expect("Failed to read line");

        // Rust allows shadowing of variables and this is common practice
        // for conversion within types.
        // Expect crashes the program upon error. Match expression allows
        // for error handling.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too less 😅"),
            Ordering::Greater => println!("too big 😭"),
            Ordering::Equal => {
                println!("You guessed right!🎊");
                break;
            }
        }
    }
}
