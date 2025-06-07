// Problem 1:
/*
Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers.
N will be a user-defined input that your program will take.

For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.
Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.
Finally, calculate the difference as 225 - 55 = 170.
*/

fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");

    let mut square_of_sum = 0;
    let mut sum_of_squares = 0;

    square_of_sum = square_sum(n);
    sum_of_squares = sum_squares(n);
    println!("{}", square_of_sum);
    println!("{}", sum_of_squares);

    let difference = square_of_sum - sum_of_squares;
    println!("{}", difference);
}

fn square_sum(num: i32) -> i64 {
    let mut sum: i32 = 0;
    for x in 1..(num + 1) {
        sum += x;
    }
    let res: i64 = (sum * sum) as i64;
    res
}

fn sum_squares(num: i32) -> i64 {
    let mut res: i64 = 0;
    for x in 1..(num + 1) {
        res += (x * x) as i64;
    }
    res
}
