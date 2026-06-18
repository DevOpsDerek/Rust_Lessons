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
    if left >= right {
        left
    } else {
        right
    }
}

/// SOLUTION: Use a `where` clause to require both `Display` and `Debug` behavior.
pub fn exercise_3<T, U>(label: T, value: U) -> String
where
    T: Display,
    U: Debug,
{
    format!("Display: {label} | Debug: {:?}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_describe_animal() {
        let cat = Animal {
            name: "Whiskers".to_string(),
            kind: "cat".to_string(),
        };
        let result = exercise_1(&cat);
        assert!(result.contains("Whiskers"));
        assert!(result.contains("cat"));
    }

    #[test]
    fn test_exercise_2_max_integers() {
        assert_eq!(exercise_2(3, 7), 7);
        assert_eq!(exercise_2(10, 2), 10);
        assert_eq!(exercise_2(5, 5), 5);
    }

    #[test]
    fn test_exercise_2_max_floats() {
        assert_eq!(exercise_2(1.5_f64, 2.5_f64), 2.5);
    }

    #[test]
    fn test_exercise_3_contains_label_and_value() {
        let result = exercise_3("score", 42u32);
        assert!(result.contains("score"));
        assert!(result.contains("42"));
    }
}
