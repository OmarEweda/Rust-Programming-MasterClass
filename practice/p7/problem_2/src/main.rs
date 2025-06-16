// Problem 2: Complete the function signature.


fn calculate_square(num: i32) -> Result<f32, String> {
    if num >= 0 {
        let result = (num * num) as f32;
        println!("The square of {} is: {}", num, result);
        return Ok(result);
    } else {
        return Err("Negative number provided".to_string());
    }
}

fn main() {
    let number = 7;
    if let Err(e) = calculate_square(number) {
        println!("Error: {e}");
    }
}
