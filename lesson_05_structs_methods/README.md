# Lesson 05: Structs & Methods

        ## What you'll learn

        - struct
- impl blocks
- associated functions
- derive Debug
- tuple structs

        ## Key concepts

        Structs let you group related data into a named type. Instead of passing around many separate values, you can model a real concept such as a rectangle, a user, or a point. Rust makes this pleasant with field init shorthand and traits like `Debug` for quick printing during learning.

        Methods live inside `impl` blocks and give behavior to your structs. Associated functions, such as `new`, do not take `self` and are often used as constructors. Tuple structs are a lighter variant when names are less important than grouping values together.

        ## How to run

        ```bash
        cargo run
        ```

        ## Exercises

        1. Define a `Book` struct with title and page count, then create one in a function.
2. Add a method to a struct that returns whether one rectangle can contain another.
3. Create a tuple struct for RGB values and return its average intensity.

        ## Tips / common mistakes

        - Read compiler error messages slowly; they are usually very specific.
        - If ownership or types feel confusing, print smaller intermediate values.
        - Compare `src/exercises.rs` and `src/solutions.rs` only after attempting the task yourself.
