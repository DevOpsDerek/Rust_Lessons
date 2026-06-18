#![allow(unused_variables)]

mod exercises;
mod solutions;

// LEARN: `main` is the entry point for a binary crate.
fn main() {
    // LEARN: `println!` is a macro, which is why it ends with `!`.
    println!("Lesson 01: Hello World & Cargo");
    println!("--------------------------------");

    // LEARN: String literals like this live in the compiled program.
    let lesson_name = "Hello World & Cargo";
    println!("Current lesson: {lesson_name}");

    // LEARN: Calling a helper function keeps `main` readable.
    show_cargo_commands();

    // LEARN: Functions can return owned `String` values.
    let greeting = build_greeting("Rust learner");
    println!("Greeting from a helper function: {greeting}");

    // LEARN: Expression blocks can calculate values without a semicolon.
    let excitement_level = {
        let base = 2;
        base + 3
    };
    println!("Excitement level: {excitement_level}");

    // LEARN: Formatting placeholders insert values into output.
    println!(
        "A Rust binary starts in `main`, source code lives in `src/main.rs`, and Cargo reads `Cargo.toml`."
    );

    // LEARN: Reusing a helper shows how code can be composed from small pieces.
    repeat_message("Practice with cargo run to build confidence.", 2);
}

// LEARN: Functions can take borrowed string slices with `&str`.
fn build_greeting(name: &str) -> String {
    format!("Hello, {name}! Welcome to Rust.")
}

fn show_cargo_commands() {
    println!("Useful Cargo commands:");
    println!("- cargo new project_name   -> creates a new project");
    println!("- cargo run                -> builds and runs the program");
    println!("- cargo build              -> builds without running");
    println!("- cargo check              -> type-checks quickly");
}

fn repeat_message(message: &str, times: usize) {
    // LEARN: Ranges like `0..times` produce a sequence of numbers.
    for index in 0..times {
        println!("Repeat #{index}: {message}");
    }
}
