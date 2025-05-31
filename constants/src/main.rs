fn main() {
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
