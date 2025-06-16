fn main() {
    /*
     * Casting mutable references to immutable references is safe in Rust.
     */
    let mut x = 5;
    let y: &mut i32 = &mut x;
    let z: &i32 = y; // Casting mutable reference to immutable reference is safe
    println!("Value of z: {}", z);

    /*
     * Casting immutable references to mutable references is unsafe in Rust.
     */
    let a = 10;
    let b: &i32 = &a;
    // let c: &mut i32 = b; // This line would cause a compile-time error because casting immutable reference to mutable reference is unsafe
    // Uncommenting the above line would result in a compilation error
    println!("Value of b: {}", b);

    // Assigning of references
    let mut num = 20;
    let mut num2 = 30;
    let ref_num: &i32 = &num; // Immutable reference
    let mut_ref_num: &mut i32 = &mut num2; // Mutable reference
    println!("Value of ref_num: {}", ref_num);
    println!("Value of mut_ref_num before change: {}", mut_ref_num);
    *mut_ref_num += 5; // Changing the value through mutable reference
    println!("Value of mut_ref_num after change: {}", mut_ref_num);
}
