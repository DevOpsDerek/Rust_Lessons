# Lesson 01: Hello World & Cargo

        ## What you'll learn

        - println! macro
- cargo new / cargo run / cargo build
- main function
- basic Rust file layout

        ## Key concepts

        Rust programs begin in a `main` function, and even the smallest example teaches useful ideas: macros look different from functions, source files live in `src/`, and Cargo manages building and running your code. In this lesson you will see how a tiny program is still a complete Rust crate.

        Cargo is Rust's build tool and package manager. It creates project structure, tracks metadata in `Cargo.toml`, and gives you simple commands like `cargo run` for development and `cargo build` for producing binaries. Learning this workflow early makes every later lesson easier.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Print your name and favorite programming topic using two separate `println!` calls.
2. Write a helper function that returns a greeting string for a provided name.
3. Build a short program summary string that mentions `cargo run` and `cargo build`.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
