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

/// SOLUTION: Call the trait method and return its string.
pub fn exercise_1(animal: &Animal) -> String {
    animal.describe()
}

/// SOLUTION: A generic function can compare any `PartialOrd` type.
pub fn exercise_2<T: PartialOrd>(left: T, right: T) -> T {
    if left >= right { left } else { right }
}

/// SOLUTION: Use a `where` clause to require both `Display` and `Debug` behavior.
pub fn exercise_3<T, U>(label: T, value: U) -> String
where
    T: Display,
    U: Debug,
{
    format!("Display: {label} | Debug: {:?}", value)
}
