fn main() {
    /*
     * Functions are a way to encapsulate reusable code.
     * They can take parameters and return values.
     */
    basic_func(); // Call the basic function

    func_with_params("Omar", 69);

    println!("{}", func_with_inputs_outputs(5, 20));

    let (sum, product) = func_with_multi_return(5, 10);
    println!("Sum: {}, Product: {}", sum, product);

    // code block
    let result = {
        let x = "10";
        let y = "20";
        format!("{}{}", x, y)
    };

    println!("Result from code block: {}", result);

    // User Inputs
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!(
        "You entered: {}",
        input.trim().parse::<i32>().expect("Failed to parse input")
    );
}

fn basic_func() {
    println!("This is a basic function in Rust!");
}

fn func_with_params(name: &str, age: i32) {
    println!("Hello, my name is {} and I am {} years old.", name, age);
}

fn func_with_inputs_outputs(num1: i32, num2: i32) -> i32 {
    // This function takes two integers and returns their sum
    num1 * num2
}

fn func_with_multi_return(num1: i32, num2: i32) -> (i32, i32) {
    // This function takes two integers and returns a tuple of their sum and product
    (num1 + num2, num1 * num2)
}
