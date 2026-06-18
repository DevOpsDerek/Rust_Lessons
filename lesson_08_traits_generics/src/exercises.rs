#![allow(dead_code)]

use std::fmt::{Debug, Display};

pub trait Describe {
    fn describe(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Animal {
    pub name: String,
    pub kind: String,
}

impl Describe for Animal {
    fn describe(&self) -> String {
        format!("{} is a {}", self.name, self.kind)
    }
}

/// Implement a trait named `Describe` for `Animal` and return the description string.
pub fn exercise_1(animal: &Animal) -> String {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Return the larger of two values using a generic function bounded by `PartialOrd`.
pub fn exercise_2<T: PartialOrd>(left: T, right: T) -> T {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Return a string that combines `Display` and `Debug` output using a `where` clause.
pub fn exercise_3<T, U>(label: T, value: U) -> String where T: Display, U: Debug {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}
