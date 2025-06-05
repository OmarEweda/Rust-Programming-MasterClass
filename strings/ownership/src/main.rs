fn main() {
    /*
     * String construction and ownership example
     */

    let s1: String = String::from("Hello");
    let s2: &str = "world";
    let s3 = s1 + s2; // s1 is moved to s3, and s2 is borrowed
    println!("{}", s3); // s3 now owns the concatenated string
    println!("s2 valid: {}", s2); // s2 is still valid because it is a borrowed string slice

    let str1: String = String::from("Hello");
    let str2: String = String::from(", world");
    let str3 = str1 + &str2; // str1 is moved to str3, and str2 is borrowed
    println!("{}", str3); // str3 now owns the concatenated string

    let str_1 = String::from("Hello");
    let str_2 = String::from(" world");
    let str_3 = String::from(" from Rust");
    let str_4 = format!("{}{}{}", str_1, str_2, str_3); // use format! for multi-string concatenation
    println!("{}", str_4); // str_4 now owns the concatenated string
}
