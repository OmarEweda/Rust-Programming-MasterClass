// Problem 5:

/*
A Pythagorean triple consists of three positive integers a, b, and c, satisfying the condition a^2 + b^2 = c^2.
These triples are commonly written as (a, b, c), and a well-known example is (3, 4, 5).

Write a program that computes the Pythagorean triplet such that a < b < c and a + b + c = 1000.
*/

fn find_pythagorean_triplet(sum: i32) -> Option<(i32, i32, i32)> {
    for a in 1..sum {
        for b in (a + 1)..sum {
            let c = sum - a - b;
            if c > b && a * a + b * b == c * c {
                return Some((a, b, c));
            }
        }
    }
    None
}

fn main() {
    match find_pythagorean_triplet(1000) {
        Some((a, b, c)) => println!("Pythagorean triplet: ({}, {}, {})", a, b, c),
        None => println!("No triplet found."),
    }
}
