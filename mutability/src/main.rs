fn main() {
    /*
     * Mutable references allow you to change the value they point to.
     * They are created using the `&mut` keyword.
     * You can only have one mutable reference to a value in a particular scope.
     * Immutable references allow you to read the value they point to.
     * They are created using the `&` keyword.
     * You can have multiple immutable references to a value in a particular scope.
     * Mutable references and immutable references cannot coexist in the same scope.
     */

    // Multiple mutable references are not allowed in the same scope.
    let mut heap_num: Vec<i32> = vec![1, 2, 3];
    let ref1 = &mut heap_num; // Mutable reference to heap_num
    println!("Mutable reference: {:?}", ref1);
    // Mutable reference ends here, so we can now create immutable references

    // Immutable references can coexist with each other, but not with mutable references.
    let ref3 = &heap_num; // Immutable reference to heap_num
    let ref4 = &heap_num; // Another immutable reference to heap_num
    println!("Immutable references: {:?}, {:?}", ref3, ref4);

    // Mutable and Immutable references cannot coexist in the same scope.

    // Uncommenting the next line will cause a compile-time error
    // let ref5 = &mut heap_num; // Mutable reference to heap_num while immutable references exist
    // println!("Mutable reference: {:?}, Immutable reference: {:?}", ref5, ref3);

    // Scope of references
    {
        let mut local_num = 10;
        {
            let ref6 = &mut local_num; // Mutable reference to local_num
            *ref6 += 5; // Modify the value through the mutable reference
            println!("Modified local_num: {}", local_num);
        } // Mutable reference goes out of scope here, allowing local_num to be used again

        let ref7 = &local_num; // Immutable reference to local_num
        println!("Immutable reference after modification: {}", ref7);
    } // Mutable reference, local_num goes out of scope here

}
