// Problem 6: Write a function that implements the logic,
// 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'
// Note: This means that if you 17 or older, you implicitly have permission. 

fn can_see_movie(age: i32, permission: bool) -> bool {
    if age >= 17 {
        true
    } else if age >= 13 && permission {
        true
    } else {
        false
    }
}

fn main() {
    println!("John who is 18, can see the move: {}", can_see_movie(17, true));
}