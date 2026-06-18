#![allow(dead_code)]

/// SOLUTION: Demonstrate immutable and mutable bindings.
pub fn exercise_1() {
    let language = "Rust";
    let mut attempts = 1;
    attempts += 2;
    println!("Learning {language} after {attempts} attempts.");
}

/// SOLUTION: Convert a `u8` into `i32` before adding.
pub fn exercise_2(left: i32, right: u8) -> i32 {
    left + i32::from(right)
}

/// SOLUTION: Return an owned string that has been extended.
pub fn exercise_3() -> String {
    let mut message = String::from("Hello");
    message.push_str(" from Rust");
    message
}
