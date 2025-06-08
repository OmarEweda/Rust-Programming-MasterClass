fn main() {
    /*
     * 1. Immutable binding of an immutable reference
     * You can read the value through the reference, but cannot modify it or reassign the binding.
     */
    let a = 15; // Immutable variable
    let b = &a; // Immutable binding to an immutable reference
    println!("1. b points to: {}", b);
    // *b += 1;          // Error: cannot mutate through immutable reference
    // b = &x;           // Error: cannot reassign immutable binding

    /*
     * 2. Mutable binding of an immutable reference
     * You can reassign the reference to point to another value, but cannot modify the pointed value.
     */
    let a = 15;
    let c = 20;
    let mut d = &a; // Mutable binding to an immutable reference
    println!("2. d points to: {}", d);
    d = &c; // Reassignment allowed
    println!("2. d now points to: {}", d);
    // *d += 1;          // Error: cannot mutate through immutable reference

    /*
     * 3. Immutable binding of a mutable reference
     * You can mutate the value it points to, but you can't reassign the binding.
     */
    let mut x = 30; // Mutable variable required for mutable reference
    let e = &mut x; // Immutable binding to a mutable reference
    *e += 5; // Allowed: can modify through e
    println!("3. x after mutation through e: {}", x);
    // e = &mut z;       // Error: cannot reassign immutable binding

    /*
     * 4. Mutable binding of a mutable reference
     * You can mutate the value it points to and reassign the binding to point elsewhere.
     */
    let mut x = 100;
    let mut z = 200;
    let mut f = &mut x; // Mutable binding to mutable reference
    *f += 50; // Modify the value of x through f
    println!("4. f points to x, value now: {}", f);
    f = &mut z; // Reassign to point to z
    *f += 25; // Modify the value of z through f
    println!("4. f now points to z, value now: {}", f);
}
