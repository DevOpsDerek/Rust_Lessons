# Lesson 02: Variables, Types & Mutability

        ## What you'll learn

        - let and mut
- type inference
- scalar types
- string literals vs String

        ## Key concepts

        Rust variables are immutable by default, which means you must opt in to change with `mut`. This encourages you to think carefully about which values should change and which should stay stable. The language also infers many types automatically, so you often get clarity without extra type annotations.

        You will also meet Rust's scalar types, such as integers, floats, booleans, and characters. Finally, you will compare string literals like `"hello"`, which are fixed slices baked into the program, with `String`, which owns heap-allocated text that can grow and change.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Create immutable and mutable variables, then print how their values differ over time.
2. Write a function that returns the sum of an `i32` and a `u8` after converting types safely.
3. Build a `String` from a string literal and append extra text to it.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
