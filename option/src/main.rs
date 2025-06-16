/*
 * option in Rust is a powerful feature that allows you to represent a value that may or may not be present.
 * It is defined in the standard library as an enum with two variants: Some(T) and None.
 * This feature is particularly useful for handling cases where a value might be absent, such as when dealing with optional parameters or results that may not always yield a value.
 */

//enum Option<T> {
//     Some(T),
//     None,
// }

struct Student {
    name: String,
    grade: Option<i32>,
}

fn get_grade(student_name: &str, student_db: &Vec<Student>) -> Option<i32> {
    for student in student_db {
        if student.name == student_name {
            return student.grade;
        }
    }
    None
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
// The Result enum is used for functions that can return a value or an error.
// It is defined in the standard library as an enum with two variants: Ok(T) and Err(E).
// The Ok variant contains a value of type T, while the Err variant contains an error of type E.
// The Result enum is particularly useful for error handling in Rust, allowing you to propagate errors without panicking.

fn check_student(student_name: &str, student_db: &Vec<Student>) -> Result<(), String> {
    for student in student_db {
        if student.name == student_name {
            return Ok(());
        }
    }
    Err(format!("Student {} not found", student_name))
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Alice"),
            grade: Some(90),
        },
        Student {
            name: String::from("Bob"),
            grade: None,
        },
        Student {
            name: String::from("Charlie"),
            grade: Some(85),
        },
    ];
    let student_name = String::from("Alice");
    let student_status = check_student(&student_name, &student_db);

    match student_status {
        Ok(_) => {
            let grade = get_grade(&student_name, &student_db);
            match grade {
                Some(g) => println!("Grade for {}: {}", student_name, g),
                None => println!("No grade available for {}", student_name),
            }
        }
        Err(error_message) => {
            println!("{error_message}");
        }
    }
}
