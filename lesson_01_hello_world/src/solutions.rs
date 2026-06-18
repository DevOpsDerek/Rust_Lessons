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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_2_greeting_contains_name() {
        let result = exercise_2("Alice");
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_exercise_2_greeting_is_nonempty() {
        assert!(!exercise_2("Bob").is_empty());
    }

    #[test]
    fn test_exercise_3_mentions_cargo_run() {
        let result = exercise_3();
        assert!(result.to_lowercase().contains("cargo run"));
    }

    #[test]
    fn test_exercise_3_mentions_cargo_build() {
        let result = exercise_3();
        assert!(result.to_lowercase().contains("cargo build"));
    }
}
