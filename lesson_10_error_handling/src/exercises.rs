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

/// Return the first item from a slice as `Option<&str>`.
pub fn exercise_1<'a>(values: &'a [&'a str]) -> Option<&'a str> {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Parse two strings into `i32`, add them, and return `Result<i32, ParseNumberError>`.
pub fn exercise_2(left: &str, right: &str) -> Result<i32, ParseNumberError> {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Match on a `Result<i32, ParseNumberError>` and return a friendly `String`.
pub fn exercise_3(result: Result<i32, ParseNumberError>) -> String {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}
