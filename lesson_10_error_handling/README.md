# Lesson 10: Error Handling

        ## What you'll learn

        - Option<T>
- Result<T, E>
- ? operator
- unwrap/expect
- custom error types
- matching on errors

        ## Key concepts

        Rust treats recoverable problems as values, not surprises. `Option<T>` represents the presence or absence of a value, while `Result<T, E>` represents either success or a recoverable error. Because these types are part of normal control flow, the compiler helps you remember to deal with missing data and failure cases.

        This lesson also introduces the `?` operator, which passes errors upward in a concise way when your function already returns `Result`. You will see `unwrap` and `expect`, but you will also learn why they should be used carefully. Finally, a small custom error type shows how your own code can communicate failures clearly.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Return `Option<&str>` with the first item in a slice, or `None` when the slice is empty.
2. Parse two numbers from strings and return their sum as `Result<i32, ParseNumberError>`.
3. Match on a `Result` and return a user-friendly message string.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
