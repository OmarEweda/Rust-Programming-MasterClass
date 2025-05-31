fn main() {
    /*
     * Tuples in Rust are fixed-size collections of elements that can be of different types.
     * They are defined using parentheses and can hold a mix of types.
     */

    let my_tuble: (&str, i32, f64) = ("Hello, Tuples!", 420, 3.14);
    println!("{}", my_tuble.0); // Accessing the first element
    println!("{}", my_tuble.1); // Accessing the second element
    println!("{}", my_tuble.2); // Accessing the third element

    // Tuples can also be destructured meaning you can assign their values to variables directly
    let (greetings, number, pi) = my_tuble;
    println!("{}", greetings); // "Hello, Tuples!"
    println!("{}", number); // 420
    println!("{}", pi); // 3.14

    let greetings = my_tuble.0; // Accessing the first element again
    println!("{}", greetings); // "Hello, Tuples!"

    // Tuples can be nested
    let nested_tuple = (my_tuble, 4, 5.0, (3.2, 4));
    println!("{}", nested_tuple.0 .0); // Accessing the first element of the nested tuple
    println!("{}", nested_tuple.0 .1); // Accessing the second element of the nested tuple
    println!("{}", nested_tuple.0 .2); // Accessing the third element of the nested tuple
    println!("{}", nested_tuple.1); // Accessing the second element of the outer tuple
    println!("{}", nested_tuple.2); // Accessing the third element of the outer tuple
    println!("{}", nested_tuple.3 .0); // Accessing the first element of the innermost tuple
    println!("{}", nested_tuple.3 .1); // Accessing the second element of the innermost tuple

    // EMpty tuple
    let empty_tuple: () = ();
    println!("{:?}", empty_tuple); // Prints: ()
}
