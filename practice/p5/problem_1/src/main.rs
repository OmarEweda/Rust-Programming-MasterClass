// Problem 1: The elements in the vector must be of the same type.
// In this exercise, we will look at a way-out for storing elements of different types in a vector.

#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}

fn main() {
    let some_val = vec![Value::Integer(12), Value::Float(15.5)];

    for i in some_val {
        match i {
            Value::Integer(num) => println!("Integer: {} ", num),
            Value::Float(num) => println!("Float: {}", num),
        }
    }
}
