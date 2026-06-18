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
    let right_number: i32 = right.parse().map_err(|source| ParseNumberError { source })?;
    Ok(left_number + right_number)
}

/// SOLUTION: Convert success and error cases into a friendly string.
pub fn exercise_3(result: Result<i32, ParseNumberError>) -> String {
    match result {
        Ok(total) => format!("The total is {total}"),
        Err(error) => format!("Could not add the numbers: {error}"),
    }
}
