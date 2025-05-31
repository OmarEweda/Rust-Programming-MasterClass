fn main() {
    /*
     * String slices in Rust allow you to create a view into a portion of a string without copying the data.
     * They are represented by the `&str` type and are used to reference a string without taking ownership of it.
     * They are fixed length and immutable, meaning you cannot change the contents of a string slice.
     */

    let some_string: &str = "Hello, world!"; // Create a string slice that references a string literal
    println!("String slice: {}", some_string); // Print the string slice

    /*
     * Styring Variables in Rust are used to store data that can be changed during the execution of a program.
     * They are represented by the `String` type and are mutable, meaning you can change the contents of a string.
     * Unlike string slices, `String` is an owned type, meaning it takes ownership of the data it contains.
     */

    let mut my_string = String::from("Hello, world!"); // Create a mutable String
    my_string.push_str(" How are you?"); // Append a string slice to the String
    println!("Mutable String: {}", my_string); // Print the mutable String
    my_string.pop(); // Remove the last character from the String
    println!("After pop: {}", my_string); // Print the modified String

    println!(
        "Is my string empty?: {},
    length of my string: {},
    does my string contain 'use'?: {}",
        my_string.is_empty(),      // Check if the String is empty
        my_string.len(),           // Get the length of the String
        my_string.contains("use")  // Check if the String contains a substring
    );

    let empty_string = String::new(); // Create an empty String
    println!("Empty String: '{}'", empty_string); // Print the empty String
    let string_from_literal = "Hello, Rust!".to_string(); // Create a String from a string literal
    println!("String from literal: {}", string_from_literal); // Print the String created from a literal

    let s_1: String = "Omar".to_string(); // Create a String from a string literal
    let s_2: String = "Eweda".to_string(); // Create another String from a string literal
    let s_3: String = format!("{} {}", s_1, s_2); // Concatenate two Strings using format!
    println!("Concatenated String: {}", s_3); // Print the concatenated String
}
