#![allow(dead_code)]

/// SOLUTION: Print two simple lines of output.
pub fn exercise_1() {
    // SOLUTION: `println!` writes a line to standard output.
    println!("My name is Ferris.");
    println!("I want to learn Rust fundamentals.");
}

/// SOLUTION: Build and return a greeting string.
pub fn exercise_2(name: &str) -> String {
    // SOLUTION: `format!` creates an owned `String`.
    format!("Hello, {name}! Great job starting Rust.")
}

/// SOLUTION: Return a summary that mentions key Cargo commands.
pub fn exercise_3() -> String {
    // SOLUTION: A plain string literal can become a `String` with `.to_string()`.
    "Use cargo run to test your code and cargo build to create the binary.".to_string()
}
