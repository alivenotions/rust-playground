fn main() {
    let x = 2;
    println!("The value of x is {}", x);
    // We are shadowing the variable and changing
    // the value depending on the value of x before
    // this. According to me this can be a source
    // of confusion but it is useful for using the
    // same variable and mutating the type.
    let x = x + 5;
    println!("The value of x is {}", x);

    // if this variable is not annotated with a type
    // then it will through an error because it cannot
    // infer it.
    let y: u32 = "34".parse().expect("Not a number");

    // Scalar types
    let z: i32 = -33;
    println!(
        "The unsigned value is: {} while the signed value is: {}",
        y, z
    );

    // Tuple is a type of compound type
    // A tuple can have elements of different types
    let a_tuple_type: (u32, f64, char) = (500, 65.2, 'a');
    // tuple supports destrcuturing
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
    // Arrays are allocated on the stack. Arrays are
    // always of fixed type. If you need a collection
    // that grows or shrinks, you need a vector.
    let first_array: [u8; 4] = [1, 2, 3, 4];
    // Need to include :? inside the formatter parens to print an array.
    // TODO: Need to find why we need to do that.
    println!("The first element in the array is: {:?}", first_array);
}
