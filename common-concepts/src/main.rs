fn returns_five() -> u8 {
    5
}

fn main() {
    // These parantheses create a block with a new scope and exp_var
    // is expecting a value to be returned from that block.
    let exp_var = {
        let x = 12;
        // omitting a semi-colon turns this into an
        // expression from a statment. Unlike a function
        // you can't use a return keyword.
        x + 12
    };

    // if is an expression and not a statement in rust.
    let if_else_var = if exp_var == 20 {
        println!("Inside an if condition");
        1
    } else if exp_var == 24 {
        println!("inside if else");
        -1
    } else {
        println!("Inside an else condition");
        0
    };
    println!("The value of if_else_var: {}", if_else_var);

    println!("The value of exp_var: {:?}", exp_var);

    let return_val = another_rogue_function(70) + returns_five();
    println!("The returned val from the function: {}", return_val);

    let x = 2;
    println!("The value of x is {}", x);
    // We are shadowing the variable and changing
    // the value depending on the value of x before
    // this. According to me this can be a source
    // of confusion but it is useful for using the
    // same variable name and mutating the type.
    let x = x + 5;
    println!("The value of x is {}", x);

    // if this variable is not annotated with a type
    // then it will throw an error because it cannot
    // infer it.
    let y: u32 = "34".parse().expect("Not a number");

    // Scalar types
    // Types like signed and unsigned integers, floats,
    // chars, bool etc.
    let z: i32 = -33;
    println!(
        "The unsigned value is: {} while the signed value is: {}",
        y, z
    );

    // Tuple is a type of compound type
    // A tuple can have elements of different types
    let a_tuple_type: (u32, f64, char) = (500, 65.2, 'a');
    // tuple supports destructuring
    let (first, second, third) = a_tuple_type;
    println!(
        "The destructured vars are: {}, {}, {}",
        first, second, third
    );

    // A certain value in the tuple can be accessed
    // using the (.) operator followed by the index
    // of that value.
    let five_hundred = a_tuple_type.0;
    println!("The first value in a_tuple_type: {}", five_hundred);

    // An array type is also a compound type but it
    // only allows for homogenous types in the array.
    // Arrays are allocated on stack. If you need a
    // collection that grows/shrinks, you need a vector.
    let first_array: [u8; 4] = [1, 2, 3, 4];
    // Need to include :? inside the formatter parens to print an array.
    // TODO: Need to find why we need to do that.
    println!("The first element in the array is: {:?}", first_array);
}

fn another_rogue_function(number: u8) -> u8 {
    println!("I am a rogue function that prints whatever it wants");
    println!(
        "and oh yeah! I print the number that's passed to me: {}",
        number
    );
    42
}
