fn main() {
    /*
     * Ownership is a core concept in Rust that ensures memory safety without needing a garbage collector.
     * Each value in Rust has a single owner, and when the owner goes out of scope, the value is dropped.
     */

    // copy and move semantics
    // Primitives like integers implement the `Copy` trait, allowing them to be copied rather than moved.
    // When a value is copied, both the original and the copy can be used independently.
    // Non-Copy types, like `String`, do not implement the `Copy` trait. When such a value is assigned to another variable,
    // ownership is transferred (moved) to the new variable, and the original variable can no longer be used.
    // This behavior prevents issues like double freeing memory and ensures that resources are managed safely.
    // The following examples illustrate these concepts:

    // Primitive types like integers implement the Copy trait
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // nonprimitive types like String do not implement the Copy trait
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // s1 is cloned to s2, both are valid and usable
    println!("s1: {}, s2: {}", s1, s2);

    let s3: &String = &s2; // s3 now borrows (referenced) s2, allowing both to be used,
                           //  without changing ownership
    println!("s2: {}, s3: {}", s2, s3);

    // Vectors and other collections also follow similar ownership rules.
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2 = v1.clone(); // v1 is cloned to v2, both are valid and usable
    println!("v1: {:?}, v2: {:?}", v1, v2);

    // scopes and ownership
    // Ownership is tied to the scope in which a variable is defined.
    // When a variable goes out of scope, its value is dropped, and the memory is freed.

    {
        let my_name: String = String::from("Omar");
        println!("My name is {}!, Inside Scope", my_name);
    } // end scope, all variables defined here are dropped (ownership is released)

    // println!("My name is {}!", my_name); // This line is outside the scope of my_name, so it cannot be used here.

    // ownership and functions
    // When passing ownership to a function, the function takes ownership of the value.

    let stack_num: i32 = 10;
    let mut heap_num: Vec<i32> = vec![1, 2, 3];
    stack_function(stack_num); // stack_num is copied (ownership DOESNOT change), so it can still be used after this function call
    println!("Stack number after function call: {}", stack_num);

    reference_heap_function(&mut heap_num);
    println!("Vector values after function call: {:?}", heap_num);

    heap_function(heap_num); // heap_num is moved (ownership changes), so it cannot be used after this function call
                             // println!("Heap number after function call: {:?}", heap_num); // Uncommenting this line will cause an error because heap_num has been moved
}

// The function copies the value of num, so it does not take ownership of it.
// The original variable keeps its value, and the function can modify the copied value without affecting the original.
// This is because i32 is a primitive type that implements the Copy trait.
fn stack_function(mut num: i32) {
    println!("Original copied value: {}", num);
    num = 56;
    println!("Copied value has been updated to: {}", num);
}

// The function takes ownership of the vector num, so it cannot be used after this function call.
// The vector is stored on the heap, and when the function is called, ownership is transferred to the function.
// After the function call, the original variable cannot be used because it has been moved.
fn heap_function(mut num: Vec<i32>) {
    num.push(4);
    println!("Moved Vector value has been updated to: {:?}", num);
}

//
fn reference_heap_function(num: &mut Vec<i32>) {
    num.push(4);
    println!("Refernced Vector value has been updated to: {:?}", num);
}
