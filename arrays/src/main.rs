fn main() {
    /*
     * Arrays in Rust are fixed-size collections of elements of the same type.
     * They are defined using square brackets and can hold a specific number of elements.
     */

    let mut my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", my_array); // Accessing the entire array

    my_array[4] = 10;
    println!("{:?}", my_array); // After modifying the last element

    // initializing an array with the same value
    let initializing_array: [i32; 5] = [0; 5];
    println!("{:?}", initializing_array);

    // String array
    let mut string_array: [&str; 3] = ["Hello", "World", "Rust"];
    println!("{:?}", string_array); // Accessing the string array
    string_array[2] = "Arrays";
    println!("{:?}", string_array); // After modifying the last element

    //slicing an array is a way to create a view into a portion of the array like a subarray
    let mut slice = &my_array[0..3]; // Slicing the first three elements
    println!("{:?}", slice); // Prints: [1, 2, 3]
    slice = &my_array[1..4]; // Slicing from the second to the fourth element
    println!("{:?}", slice); // Prints: [2, 3, 4]

    // Array length
    println!("Number of elements in my_array: {}", my_array.len());

    // Array functions
    println!(
        "The array occupyies {} bytes",
        std::mem::size_of_val(&my_array)
    );

    let check_index = my_array.get(100); // Safe way to access an element
    match check_index {
        Some(value) => println!("Element at index 2: {}", value),
        None => println!("Index out of bounds"),
    }
}
