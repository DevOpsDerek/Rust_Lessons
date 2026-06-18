#![allow(dead_code)]

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseNumberError {
    pub source: ParseIntError,
}

impl fmt::Display for ParseNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse number: {}", self.source)
    }
}

impl Error for ParseNumberError {}

/// SOLUTION: `first()` already returns `Option<&T>`.
pub fn exercise_1<'a>(values: &'a [&'a str]) -> Option<&'a str> {
    values.first().copied()
}

/// SOLUTION: Map parse errors into a custom error and use `?`.
pub fn exercise_2(left: &str, right: &str) -> Result<i32, ParseNumberError> {
    let left_number: i32 = left.parse().map_err(|source| ParseNumberError { source })?;
    let right_number: i32 = right
        .parse()
        .map_err(|source| ParseNumberError { source })?;
    Ok(left_number + right_number)
}

/// SOLUTION: Convert success and error cases into a friendly string.
pub fn exercise_3(result: Result<i32, ParseNumberError>) -> String {
    match result {
        Ok(total) => format!("The total is {total}"),
        Err(error) => format!("Could not add the numbers: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_returns_first_element() {
        assert_eq!(exercise_1(&["a", "b", "c"]), Some("a"));
    }

    #[test]
    fn test_exercise_1_empty_slice_returns_none() {
        assert_eq!(exercise_1(&[]), None);
    }

    #[test]
    fn test_exercise_2_adds_valid_numbers() {
        assert_eq!(exercise_2("10", "32").unwrap(), 42);
        assert_eq!(exercise_2("0", "0").unwrap(), 0);
    }

    #[test]
    fn test_exercise_2_invalid_input_returns_err() {
        assert!(exercise_2("abc", "1").is_err());
        assert!(exercise_2("1", "xyz").is_err());
    }

    #[test]
    fn test_exercise_3_ok_result_message() {
        let ok_result: Result<i32, ParseNumberError> = Ok(99);
        let msg = exercise_3(ok_result);
        assert!(msg.contains("99"));
    }

    #[test]
    fn test_exercise_3_err_result_message() {
        let err_result = exercise_2("bad", "input");
        let msg = exercise_3(err_result);
        assert!(!msg.is_empty());
        assert!(!msg.contains("total"));
    }
}
