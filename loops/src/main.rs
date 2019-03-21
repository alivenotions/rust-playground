fn main() {
    let mut val = 0;
    let broken_loop_val = loop {
        // break can also return a value that becomes the returned
        // value of this loop. This can be used to pass the result
        // of this loop to the rest of the code.
        if val > 4 {
            break val;
        }
        val = val + 1;
        println!("the value of val is: {}", val);
        println!("again");
    };
    println!("The broken_loop_val: {}", broken_loop_val);
    assert_eq!(broken_loop_val, 5);

    while val < 10 {
        println!("the value of val is: {}", val);
        val = val + 1;
    }

    let arr = [1, 2, 3, 4];
    for element in arr.iter() {
        println!("The element in arr: {}", element);
    }

    // The range includes the first element but not the last.
    for element in (1..4).rev() {
        println!("The element in arr: {}", element);
    }

    println!("The sixth fibonacci num: {}", get_nth_fibonacci(6));
}

fn get_nth_fibonacci(num: u32) -> u32 {
    let mut first = 1;
    let mut second = 1;

    let mut i = 3;
    let result = if num == 1 {
        1
    } else if num == 2 {
        1
    } else {
        let mut sum = 0;
        while i <= num {
            sum = first + second;
            first = second;
            second = sum;
            i = i + 1;
        }
        sum
    };
    result
}
