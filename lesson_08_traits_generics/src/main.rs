#![allow(unused_variables)]

mod exercises;
mod solutions;

use std::fmt::{Debug, Display};

trait Summary {
    fn summary(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
struct Article {
    title: String,
    author: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn main() {
    println!("Lesson 08: Traits & Generics");
    println!("-----------------------------");

    let article = Article {
        title: String::from("Learning Rust"),
        author: String::from("Ava"),
    };
    println!("Summary: {}", article.summary());

    let bigger_number = largest(8, 12);
    let bigger_float = largest(2.5, 1.5);
    println!("Largest number: {bigger_number}");
    println!("Largest float: {bigger_float}");

    show_display_and_debug("rust", &article);

    let cloned = article.clone();
    println!("Clone equals original? {}", cloned == article);
}

fn largest<T: PartialOrd>(left: T, right: T) -> T {
    if left >= right {
        left
    } else {
        right
    }
}

fn show_display_and_debug<T, U>(label: T, value: &U)
where
    T: Display,
    U: Debug,
{
    println!("Display label: {label}");
    println!("Debug value: {:?}", value);
}
