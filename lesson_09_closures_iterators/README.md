# Lesson 09: Closures & Iterators

        ## What you'll learn

        - closure syntax
- Fn/FnMut/FnOnce
- map/filter/collect/fold
- iterator chaining

        ## Key concepts

        Closures are small anonymous functions that can capture values from their environment. They are useful when you need short bits of behavior close to where they are used, especially with iterator methods. Rust classifies closures by how they capture and use values, which leads to the `Fn`, `FnMut`, and `FnOnce` traits.

        Iterators process sequences lazily, one step at a time. Instead of writing every loop manually, you can chain adaptors like `map`, `filter`, and `collect` to describe what should happen to data. This style often reads like a pipeline and keeps logic focused.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Write a closure that adds a captured bonus to each score in a vector.
2. Use iterator chaining to filter even numbers and double them.
3. Use `fold` to compute the total character count of several words.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
