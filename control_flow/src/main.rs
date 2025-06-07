fn main() {
    /*
     * control_flow in Rust allows you to execute code based on certain conditions or to repeat code multiple times.
     * You can also use loops like `for`, `while`, and `loop` to repeat code.
     * Additionally, Rust provides pattern matching with the `match` statement, which is a powerful way to handle different cases.
     * This example demonstrates the use of these control flow constructs.
     */

    // loop example

    loop {
        println!("This is an infinite loop. Unless you break it, it will run forever.");
        break;
    }

    'outer: loop {
        println!("This is an outer loop.");
        loop {
            println!("This is an inner loop.");
            break 'outer; // Breaks out of the outer loop
        }
        println!("This line will not be printed because we broke out of the outer loop.");
    }

    let mut a = loop {
        break 42; // This loop will break and return 42
    };
    println!("The value of a is: {}", a);

    //for loop example
    let vec: Vec<i32> = vec![45, 55, 65, 75, 85];

    for i in vec {
        println!("The value of i is: {}", i);
    }

    // while loop example
    let mut count = 0;
    while count < 10 {
        count += 1;
        println!("Count is: {}", count);
    }
}
