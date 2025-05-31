fn main() {
    /*
     * Vectors in Rust are implemented as a growable array type.
     * They are stored on the heap and can dynamically resize as elements are added or removed.
     * Vectors are defined using the `Vec<T>` type, where `T` is the type of elements stored in the vector.
     */

    let mut num_vector: Vec<i32> = vec![5, 6, 7, 8, 9, 10, 11]; // Create a vector with initial values
    println!("Initial vector: {:?}", num_vector);

    num_vector[0] = 1; // Update the first element
    println!("Updated vector: {:?}", num_vector);

    // Initialize a vector with same default values
    let initialized_vector: Vec<i32> = vec![0; 10];
    println!("Initialized vector: {:?}", initialized_vector);

    // String Vector
    let mut string_vector: Vec<&str> = vec!["Hello", "World", "Rust"];
    println!("String vector: {:?}", string_vector);
    string_vector.push("Programming"); // Add a new element to the vector
    println!("Updated string vector: {:?}", string_vector);

    // Character Vector
    let char_vector: Vec<char> = vec!['a', 'b', 'c', 'd'];
    println!("Character vector: {:?}", char_vector);

    // Slicing a vector
    let mut slice_of_vector = &num_vector[1..4]; // Get a slice of the vector from index 1 to 3
    println!("Slice of vector: {:?}", slice_of_vector);
    slice_of_vector = &num_vector[2..]; // Update the slice to start from index 2
    println!("Updated slice of vector: {:?}", slice_of_vector);

    // Number of elements in a vector
    println!("Number of elements in num_vector: {}", num_vector.len());
    // Check if the vector is empty
    println!("Is num_vector empty? {}", num_vector.is_empty());

    // check if a vector contains a specific element
    let contains_five = num_vector.contains(&5);
    println!("Does num_vector contain 5? {}", contains_five);

    // check if index is out of bounds
    let check_index = num_vector.get(10);
    println!("Element at index 10: {:?}", check_index);

    // Add elements to the vector
    num_vector.push(12);
    println!("Vector after adding 12: {:?}", num_vector);
    num_vector.push(13);
    println!("Vector after adding 13: {:?}", num_vector);

    // Remove the last element from the vector
    num_vector.remove(0); // Remove the first element
    println!("Vector after removing first element: {:?}", num_vector);
    num_vector.pop(); // Remove the last element
    println!("Vector after popping last element: {:?}", num_vector);

    // Clear the vector
    num_vector.clear();
    println!("Vector after clearing: {:?}", num_vector);
    // Check if the vector is empty after clearing
    println!(
        "Is num_vector empty after clearing? {}",
        num_vector.is_empty()
    );
}
