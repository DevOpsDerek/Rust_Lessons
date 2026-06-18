#![allow(unused_variables)]

mod exercises;
mod solutions;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // LEARN: Associated functions do not take `self`.
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    // LEARN: `&self` borrows the struct so the method can read it.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)]
struct Color(u8, u8, u8);

fn main() {
    println!("Lesson 05: Structs & Methods");
    println!("-----------------------------");

    let rect = Rectangle::new(30, 20);
    let small = Rectangle::new(10, 5);
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Can hold small rectangle? {}", rect.can_hold(&small));

    let favorite = Color(120, 200, 255);
    println!("Tuple struct color: {:?}", favorite);
    println!(
        "RGB channels: ({}, {}, {})",
        favorite.0, favorite.1, favorite.2
    );

    let user = build_user(String::from("Ava"), 3);
    println!(
        "User summary: {} has completed {} lessons.",
        user.name, user.completed_lessons
    );
}

#[derive(Debug)]
struct UserProgress {
    name: String,
    completed_lessons: u8,
}

fn build_user(name: String, completed_lessons: u8) -> UserProgress {
    UserProgress {
        name,
        completed_lessons,
    }
}
