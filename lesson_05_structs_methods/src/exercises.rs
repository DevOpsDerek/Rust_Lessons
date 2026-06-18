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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// Return a `Book` struct with a title and page count.
pub fn exercise_1(title: &str, pages: u32) -> Book {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Return whether `outer` can contain `inner` by comparing width and height.
pub fn exercise_2(outer: &Rectangle, inner: &Rectangle) -> bool {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Compute the average intensity of an RGB tuple struct and return it as `u16`.
pub fn exercise_3(color: Rgb) -> u16 {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}
