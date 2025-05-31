fn main() {
    /*
     * Variables in Rust are immutable by default, meaning once a variable is bound to a value, you cannot change that value.
     */

    let x: i32 = 5; // Declare a variable x of type i32 and assign it the value 5
    println!("Value of x: {}", x); // Print the value of x

    // x = 60; // This line will cause a compile-time error because x is immutable
    // println!("Value of x: {}", x); // This line will not be reached due to the error above

    let mut y: i32 = 10; // Declare a mutable variable y of type i32 and assign it the value 10
    println!("Value of y: {}", y); // Print the value of y
    y = 20; // Change the value of y to 20
    println!("Value of y after change: {}", y); // Print the new value of y

    /*
     * Scalar types in Rust include integers, floating-point numbers, booleans, and characters.
     * Here we demonstrate the use of some scalar types.
     */
    let a: i32 = 42; // Integer type
    let b: f64 = 3.14; // Floating-point type
    let c: bool = true; // Boolean type
    let d: char = 'R'; // Character type
    println!(
        "Integer a: {}, Float b: {}, Boolean c: {}, Character d: {}",
        a, b, c, d
    ); // Print the values of the scalar types

    println!("{}", std::i8::MAX); // Print the maximum value of i8
    println!("{}", std::u8::MAX); // Print the maximum value of u8

    println!("{}", std::i16::MAX); // Print the minimum value of i16
    println!("{}", std::u16::MAX); // Print the maximum value of u16

    println!("{}", std::i32::MAX); // Print the maximum value of i32
    println!("{}", std::u32::MAX); // Print the maximum value of u32

    println!("{}", std::i64::MAX); // Print the maximum value of i64
    println!("{}", std::u64::MAX); // Print the maximum value of u64

    println!("{}", std::f64::MAX); // Print the maximum value of f64
    println!("{}", std::f32::MAX); // Print the maximum value of f32

    let not_equals: bool = 18 != 18;
    println!("Value of condition: {}", not_equals); // Print the result of the comparison

    let c1: char = 'a'; // Declare a character variable c1
    let c2: char = '3'; // Declare another character variable c2
    let c3: char = '+'; // Declare a character variable c3
    let c4: char = '\u{290A}'; // Declare a character variable c4 with a Unicode code point
    println!("Character c1: {}, c2: {}, c3: {}, c4: {}", c1, c2, c3, c4); // Print the values of the character variables

    /*
     * Initializing multiple variables simultaneously
     */

    let (first, second, third) = (1, 2.5, true);
    println!("first: {}, second: {}, third: {}", first, second, third);

    /*
     * Readability of large numbers
     * Using underscores to improve readability of large numbers
     */

    let large_number: u64 = 1_000_000_000;
    println!("Large number: {}", large_number);

    /*
     * Integer overflow
     * Uncommenting the next line will cause a panic due to integer overflow
     */

    // let overflowed_number: u8 = 255 + 1; // This will panic at runtime
    // println!("Overflowed number: {}", overflowed_number);

    /*
     * operations on variables from different types
     * Rust does not allow implicit type conversion, so we need to explicitly convert types
     */

    let int_value: i32 = 10;
    let float_value: f64 = 5.5;
    let sum1 = int_value as f64 + float_value;
    let sum2 = int_value + float_value as i32;
    println!("Sum of int and float (int to float): {}", sum1); // explicit conversion from int to float
    println!("Sum of int and float (float to int): {}", sum2); // narrowing conversion, may lose precision
}
