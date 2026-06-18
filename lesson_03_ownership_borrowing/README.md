# Lesson 03: Ownership & Borrowing

        ## What you'll learn

        - ownership rules
- move semantics
- & and &mut references
- borrow checker
- slices

        ## Key concepts

        Ownership is one of Rust's most important ideas. Every value has one owner, and when that owner goes out of scope the value is cleaned up automatically. This gives Rust memory safety without a garbage collector, but it also means you must be aware of moves and borrows when passing data around.

        Borrowing lets you temporarily access data without taking ownership. Shared references (`&T`) allow reading, while mutable references (`&mut T`) allow changing a value when no other borrows are active. Slices build on borrowing by letting you refer to part of a collection without copying it.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Write a function that takes ownership of a `String` and returns its length.
2. Write a function that borrows a string slice and returns its first word.
3. Write a function that mutably borrows a `String` and appends a suffix.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
