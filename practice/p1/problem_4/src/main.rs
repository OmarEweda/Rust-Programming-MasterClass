// Problem 4:

/*
A palindrome is a word, verse, or sentence that reads the same backward or forward,
such as 'Able was I ere I saw Elba,' or a number like 1881.

Write a function named is_palindrome() that checks whether a given string is a palindrome or not.
The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.
*/

fn main() {
    let input = String::from("18581");
    println!(
        "It is {:?} that the given string is palindrome",
        palindrome(input)
    );
}

fn palindrome(input: String) -> bool {
    /* Your Code here */
    let length = input.len();

    for i in 0..(length / 2) {
        if input.chars().nth(i) != input.chars().nth(length - 1 - i) {
            return false;
        }
    }
    true
}
