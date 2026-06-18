#![allow(unused_variables)]

mod exercises;
mod solutions;

fn main() {
    println!("Lesson 03: Ownership & Borrowing");
    println!("--------------------------------");

    let title = String::from("ownership");
    // LEARN: Passing `title` by value moves ownership into the function.
    let title_length = take_and_measure(title);
    println!("Length after move into function: {title_length}");

    let original = String::from("borrow me");
    // LEARN: `&original` creates a shared reference, so `original` stays usable.
    let borrowed_length = measure_by_reference(&original);
    println!("Borrowed length: {borrowed_length}");
    println!("Original is still available: {original}");

    let cloned = original.clone();
    println!("Cloned copy: {cloned}");

    let mut note = String::from("Rust");
    // LEARN: `&mut` allows changing the original value through a mutable borrow.
    add_suffix(&mut note);
    println!("After mutable borrow: {note}");

    let sentence = String::from("borrow checker keeps data safe");
    let first = first_word(&sentence);
    println!("First word slice: {first}");

    // LEARN: Slices borrow part of the data instead of copying it.
    let part = &sentence[0..6];
    println!("First six bytes as a slice: {part}");
}

fn take_and_measure(text: String) -> usize {
    text.len()
}

fn measure_by_reference(text: &str) -> usize {
    text.len()
}

fn add_suffix(text: &mut String) {
    text.push_str(" language");
}

fn first_word(text: &str) -> &str {
    for (index, byte) in text.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            return &text[..index];
        }
    }
    text
}
