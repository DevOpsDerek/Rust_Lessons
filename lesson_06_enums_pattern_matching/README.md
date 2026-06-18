# Lesson 06: Enums & Pattern Matching

        ## What you'll learn

        - enum
- match
- if let
- while let
- exhaustive matching
- _ wildcard

        ## Key concepts

        Enums let a value be one of several related possibilities. Unlike many languages, Rust enums can carry data with each variant, which makes them powerful tools for representing state and outcomes. Options, results, commands, and messages are all natural fits for enums.

        Pattern matching is the natural partner to enums. `match` checks each possible shape of a value and requires you to handle every case, which helps prevent bugs. `if let` and `while let` are lighter tools when you only care about one pattern at a time.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Define an enum for traffic lights and return a string describing the next action.
2. Match on an enum that may contain a score and return a friendly message.
3. Use `while let` to drain numbers from a vector and sum them.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
