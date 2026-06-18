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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_2_adds_i32_and_u8() {
        assert_eq!(exercise_2(10, 5), 15);
        assert_eq!(exercise_2(0, 255), 255);
    }

    #[test]
    fn test_exercise_2_negative_left() {
        assert_eq!(exercise_2(-3, 3), 0);
    }

    #[test]
    fn test_exercise_3_returns_extended_string() {
        let result = exercise_3();
        assert!(result.starts_with("Hello"));
        assert!(result.contains("Rust"));
    }
}
