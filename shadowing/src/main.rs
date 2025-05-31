fn main() {
    /*
     * Shadowing in Rust allows you to declare a new variable with the same name as a previous variable, effectively "shadowing" the original variable.
     * This can be useful for transforming a value or changing its type without needing to use a new name.
     */

    let x = 5; // Declare a variable x and assign it the value 5
    println!("Value of x: {}", x); // Print the value of x
    let x = 10; // Shadow the original x with a new value (x + 1)
    println!("Value of x after shadowing: {}", x); // Print the new value of x

    let mut y = 20; // Declare a mutable variable y and assign it the value 20
    println!("Value of y: {}", y); // Print the value of y
    y = 50; // Reassign the value of y to 50
    println!("Value of y after reassignment: {}", y); // Print the new value of y
                                                      // Shadowing y with a new value
                                                      // Note: This is not a reassignment; it creates a new variable y that shadows the previous one.
    let y = y + 10; //  y now is immutable
    println!("Value of y after shadowing: {}", y); // Print the new value of y

    let y: char = 'R'; // Shadowing y with a character type
    println!("Value of y after shadowing with char: {}", y); // Print the new value of y
    let y: f64 = 3.14; // Shadowing y with a floating-point type
    println!("Value of y after shadowing with f64: {}", y); // Print the new value of y

    /*
     * In Rust, variables can be shadowed within different scopes. This means you can have a variable with the same name in an inner scope that shadows the outer variable.
     * This is useful for creating temporary variables that are only needed within a specific block of code.
     */

    let z = 100; // Declare a variable z and assign it the value 100
    {
        let z = 200; // Shadow z within this inner scope
        println!("Value of z in inner scope: {}", z); // Print the value of z in the inner scope
    }
    println!("Value of z after inner scope: {}", z); // Print the value of z after the inner scope

    /*
     * Constants in Rust are immutable values that are bound to a name and can be set to a value at compile time.
     * Constants are defined using the `const` keyword and must have a type annotation.
     * Unlike variables, constants cannot be changed once they are set, and they must always be explicitly typed.
     * Constants can be used in any scope, including global scope, and they are not limited to the block in which they are defined.
     * Constants are typically used for values that are known at compile time and do not change, such as mathematical constants or configuration values.
     */

    const MAX_POINTS: u32 = 100_000; // Define a constant MAX_POINTS of type u32 with a value of 100,000
    println!("The maximum points are: {}", MAX_POINTS); // Print the value of the constant MAX_POINTS
}
