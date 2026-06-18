#![allow(unused_variables)]

mod exercises;
mod solutions;

fn main() {
    println!("Lesson 02: Variables, Types & Mutability");
    println!("-----------------------------------------");

    // LEARN: `let` creates an immutable binding by default.
    let score = 10;
    println!("Initial score: {score}");

    // LEARN: `mut` allows a binding to change after creation.
    let mut level = 1;
    println!("Starting level: {level}");
    level += 1;
    println!("Level after increment: {level}");

    // LEARN: Rust usually infers types from how a value is used.
    let inferred_float = 3.5;
    let explicit_age: u8 = 30;
    println!("Float: {inferred_float}, age: {explicit_age}");

    // LEARN: These are common scalar types.
    let temperature_celsius: i32 = 21;
    let distance_km: u64 = 150;
    let pi_estimate: f64 = std::f64::consts::PI;
    let is_learning: bool = true;
    let grade: char = 'A';
    println!(
        "i32={temperature_celsius}, u64={distance_km}, f64={pi_estimate}, bool={is_learning}, char={grade}"
    );

    // LEARN: A string literal is an immutable borrowed slice.
    let literal = "hello";
    // LEARN: `String` owns its text and can grow.
    let mut owned = String::from(literal);
    owned.push_str(" rustaceans");
    println!("literal: {literal}");
    println!("owned String: {owned}");

    // LEARN: Shadowing lets you reuse a name for a new value.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces after shadowing: {spaces}");

    let doubled = double_number(8);
    println!("8 doubled is {doubled}");
}

fn double_number(value: i32) -> i32 {
    // LEARN: The last expression becomes the return value.
    value * 2
}
