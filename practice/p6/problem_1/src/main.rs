// Problem 1: Fix the code so that it compiles.

fn first_character(chars: &Vec<char>) -> Option<char> {
    if chars.len() > 0 {
        Some(chars[0])
    } else {
        None
    }
}

fn main() {
    let my_chars = vec!['a', 'b', 'c', 'd'];
    match first_character(&my_chars) {
        Some(charr) => println!("First character: {}", charr),
        None => println!("Empty array"),
    }
}
