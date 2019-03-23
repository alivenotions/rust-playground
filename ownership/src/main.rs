fn main() {
    let x = "Still"; // x is valid from here.
                     // can do stuff from here as x is in scope.
                     // x is stored on the stack and is popped off it when it
                     // goes out of scope. This is not very difficult to keep track of.
                     // Other complex data types like the String data type are stored
                     // on the heap and require Rust to clean it up.
    let s = String::from("Whoops!"); // this is a string literal wrapped in a string type.

    println!("The string literal: {}", x);
    println!("The string type: {}", s);

    let mut s = String::from("H");
    s.push_str("ello!");

    println!("Mutated string from string type: {}", s);

    let i1 = 5;
    let i2 = i1;
    println!("i1: {}, i2: {}", i1, i2);

    let s1 = String::from("Wow!");
    let s2 = s1;
    // The next commented line throws an error
    // because data pointed by s1 has been moved
    // to s2. At any instance, there can only be
    // one owner of a value at a time. This ensures
    // that rust frees the resources only once
    // when an owner goes out of scope.
    // println!("s1: {}", s1);
    println!("s2: {}", s2);

    let s3 = String::from("Third time");
    // Clone method call makes it explicit that this
    // is a deep clone and data on heap is creted again.
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);

    let str_to_pass = String::from("Hello World!");
    let int_to_pass = 22;

    // str_to_pass is moved to the parameter of takes_ownership.
    takes_ownership(str_to_pass);
    // int_to_pass is copied to the parameter of makes_copy.
    makes_copy(int_to_pass);
    // The following line throws an error because str_to_pass
    // is no longer in scope and has been moved to the parameter
    // of makes_copy.
    // println!("str_to_pass from main: {}", str_to_pass);
    // int_to_pass is still available because int_to_pass
    // was copied and not moved.
    println!("int_to_pass from main: {}", int_to_pass);
} // x is not valid from here onwards as it is out of the scope.

fn takes_ownership(some_string: String) {
    println!("This is some_string; {}", some_string);
}

fn makes_copy(some_integer: u32) {
    println!("This is some_integer: {}", some_integer);
}
