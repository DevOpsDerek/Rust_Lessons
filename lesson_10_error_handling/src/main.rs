#![allow(unused_variables)]

mod exercises;
mod solutions;

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct ParseLessonError {
    source: ParseIntError,
}

impl fmt::Display for ParseLessonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse lesson number: {}", self.source)
    }
}

impl Error for ParseLessonError {}

fn main() {
    println!("Lesson 10: Error Handling");
    println!("-------------------------");

    let values = ["10", "20", "30"];
    let first = values.first();
    match first {
        Some(value) => println!("First value with Option: {value}"),
        None => println!("No values found"),
    }

    match add_parsed_numbers("8", "12") {
        Ok(total) => println!("Parsed total = {total}"),
        Err(error) => println!("Error while parsing: {error}"),
    }

    // LEARN: expect() and unwrap_or() show how to safely extract Option values.
    // In real code Option comes from functions that might return nothing.
    let names = ["Alice", "Bob"];
    let first_name = names.first().expect("names list is never empty");
    println!("Value from expect: {first_name}");

    let empty: &[i32] = &[];
    let fallback = empty.first().copied().unwrap_or(0);
    println!("unwrap_or fallback example: {fallback}");
}

fn add_parsed_numbers(left: &str, right: &str) -> Result<i32, ParseLessonError> {
    // LEARN: `?` returns early if parsing fails.
    let left_number: i32 = left.parse().map_err(|source| ParseLessonError { source })?;
    let right_number: i32 = right
        .parse()
        .map_err(|source| ParseLessonError { source })?;
    Ok(left_number + right_number)
}
