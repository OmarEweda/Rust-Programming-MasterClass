fn main() {
    /*
     * Conditionals in Rust allow you to execute code based on certain conditions.
     * The most common conditional statements are `if`, `else if`, and `else`.
     */

    // Example of a simple if-else statement
    let num = 10;
    if num < 50 {
        println!("The number is less than 50.");
    } else {
        println!("The number is 50 or greater.");
    }

    // Example of if-else ladder
    let score = 85;
    let mut grade: char = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    };

    println!("Your grade is: {}", grade);

    // match statement as a conditional
    let marks = 75;
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    }
    println!("Your grade using match is: {}", grade);
}
