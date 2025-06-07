// Problem 2:

/*
Write a program to find the sum of natural numbers below a given number N, where N is provided by the user.
The sum should only include numbers that are multiples of either 3 or 5.
For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15.

Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.
The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.
*/

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let sum = natural_sum(n);
    println!("{}", sum);
}

fn natural_sum(num: i32) -> i32 {
    let mut sum: i32 = 0;
    for x in 1..num {
        if x % 3 == 0 || x % 5 == 0 {
            sum += x;
        }
    }
    sum
}
