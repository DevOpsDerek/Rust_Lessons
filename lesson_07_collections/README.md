# Lesson 07: Collections: Vec & HashMap

        ## What you'll learn

        - Vec<T>
- HashMap<K,V>
- iter/iter_mut/into_iter
- common collection methods

        ## Key concepts

        Collections store multiple values together. `Vec<T>` keeps items in order and is ideal when you want to grow a list or access items by index. Rust vectors are generic, so the same type can store integers, strings, or any other single element type.

        `HashMap<K, V>` stores key-value pairs for fast lookups. In this lesson you will practice inserting, updating, reading, and iterating over both vectors and maps. You will also compare `.iter()`, `.iter_mut()`, and `.into_iter()` so you can choose whether to borrow or consume a collection.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Create a vector, push several numbers, and return the sum.
2. Mutably iterate over a vector of scores and add a bonus to each score.
3. Build a `HashMap<String, u32>` from a list of `(name, score)` pairs.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
