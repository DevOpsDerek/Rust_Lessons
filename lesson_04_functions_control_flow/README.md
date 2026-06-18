# Lesson 04: Functions & Control Flow

        ## What you'll learn

        - fn and return types
- if/else
- loop/while/for
- break/continue
- ranges

        ## Key concepts

        Functions help you organize logic into reusable, named pieces. In Rust, functions can take typed parameters, return values, and even rely on expression blocks where the last expression becomes the result. Once you are comfortable with this style, you can write code that is both explicit and compact.

        Control flow decides which code runs and how often. Rust gives you `if` and `else` for branching, `loop` for repeated work until you choose to stop, `while` for condition-based repetition, and `for` for iterating over ranges or collections. These constructs are foundational for almost every program.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Write a function that returns `"even"` or `"odd"` for an integer.
2. Use a `loop` to count upward until a target value is reached and then return it.
3. Use a `for` loop over a range to compute the sum from 1 to `n`.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
