/*
 * option in Rust is a powerful feature that allows you to represent a value that may or may not be present.
 * It is defined in the standard library as an enum with two variants: Some(T) and None.
 * This feature is particularly useful for handling cases where a value might be absent, such as when dealing with optional parameters or results that may not always yield a value.
 */

//enum Option<T> {
//     Some(T),
//     None,
// }

struct Students {
    name: String,
    grade: Option<i32>,
}

fn get_grade(student_name: String, student_db: &Vec<Students>) -> Option<i32> {
    for student in student_db {
        if student.name == student_name {
            return student.grade;
        }
    }
    None
}

fn main() {
    let student_db = vec![
        Students {
            name: String::from("Alice"),
            grade: Some(90),
        },
        Students {
            name: String::from("Bob"),
            grade: None,
        },
        Students {
            name: String::from("Charlie"),
            grade: Some(85),
        },
    ];

    let student_name = String::from("Alice");

    match get_grade(student_name, &student_db) {
        Some(grade) => println!("Grade found: {}", grade),
        None => println!("Grade not found for the student."),
    }
}
