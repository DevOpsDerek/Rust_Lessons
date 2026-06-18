#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub pages: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    // SOLUTION: A method can inspect another struct by borrowing it.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// SOLUTION: Create and return a `Book` value.
pub fn exercise_1(title: &str, pages: u32) -> Book {
    Book { title: title.to_string(), pages }
}

/// SOLUTION: Use the rectangle method to compare sizes.
pub fn exercise_2(outer: &Rectangle, inner: &Rectangle) -> bool {
    outer.can_hold(inner)
}

/// SOLUTION: Convert channels to a wider integer type before averaging.
pub fn exercise_3(color: Rgb) -> u16 {
    (u16::from(color.0) + u16::from(color.1) + u16::from(color.2)) / 3
}
