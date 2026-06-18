# Lesson 08: Traits & Generics

        ## What you'll learn

        - trait
- impl Trait for Type
- generic functions
- where clauses
- Display/Debug/Clone/PartialEq

        ## Key concepts

        Generics let you write code that works with many types without repeating yourself. Instead of creating separate functions for `i32`, `f64`, and `String`, you can describe the shape of the behavior you need and let the compiler generate the concrete versions.

        Traits describe shared behavior. When a type implements a trait, it promises to provide certain methods or capabilities. Rust's standard traits such as `Debug`, `Clone`, and `PartialEq` are used constantly, and custom traits let you express concepts from your own programs.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Define a trait for describing an item and implement it for a struct.
2. Write a generic function that returns the larger of two comparable values.
3. Write a function with a `where` clause that prints both `Display` and `Debug` output.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
