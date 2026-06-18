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
    Book {
        title: title.to_string(),
        pages,
    }
}

/// SOLUTION: Use the rectangle method to compare sizes.
pub fn exercise_2(outer: &Rectangle, inner: &Rectangle) -> bool {
    outer.can_hold(inner)
}

/// SOLUTION: Convert channels to a wider integer type before averaging.
pub fn exercise_3(color: Rgb) -> u16 {
    (u16::from(color.0) + u16::from(color.1) + u16::from(color.2)) / 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_creates_book() {
        let book = exercise_1("Rust Programming", 500);
        assert_eq!(book.title, "Rust Programming");
        assert_eq!(book.pages, 500);
    }

    #[test]
    fn test_exercise_2_outer_holds_inner() {
        let outer = Rectangle {
            width: 10,
            height: 10,
        };
        let inner = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(exercise_2(&outer, &inner));
    }

    #[test]
    fn test_exercise_2_inner_does_not_hold_outer() {
        let outer = Rectangle {
            width: 10,
            height: 10,
        };
        let inner = Rectangle {
            width: 15,
            height: 15,
        };
        assert!(!exercise_2(&outer, &inner));
    }

    #[test]
    fn test_exercise_3_average_of_black() {
        assert_eq!(exercise_3(Rgb(0, 0, 0)), 0);
    }

    #[test]
    fn test_exercise_3_average_of_white() {
        assert_eq!(exercise_3(Rgb(255, 255, 255)), 255);
    }

    #[test]
    fn test_exercise_3_average_mixed() {
        // (0 + 150 + 255) / 3 = 135
        assert_eq!(exercise_3(Rgb(0, 150, 255)), 135);
    }
}
