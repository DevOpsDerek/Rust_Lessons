# Learn Rust

This repository is a beginner-friendly Rust curriculum with 10 hands-on lessons. Each lesson is a small standalone Cargo project with three parts: a worked `main.rs`, guided `exercises.rs`, and complete `solutions.rs`.

## Prerequisites

- Install Rust with [rustup](https://rustup.rs/)
- Verify the toolchain with `rustc --version` and `cargo --version`
- Use a text editor you are comfortable with

## Lesson roadmap

| # | Topic | Key concepts |
|---|---|---|
| 1 | Hello World & Cargo | `println!`, Cargo commands, `main`, project structure |
| 2 | Variables, Types & Mutability | `let`, `mut`, inference, scalar types, `String` |
| 3 | Ownership & Borrowing | moves, references, mutable references, slices |
| 4 | Functions & Control Flow | `fn`, return values, `if`, loops, ranges |
| 5 | Structs & Methods | `struct`, `impl`, associated functions, tuple structs |
| 6 | Enums & Pattern Matching | `enum`, `match`, `if let`, `while let` |
| 7 | Collections: Vec & HashMap | vectors, maps, iteration styles, common methods |
| 8 | Traits & Generics | traits, trait bounds, generics, `where` clauses |
| 9 | Closures & Iterators | closure capture, iterator adaptors, chaining, `fold` |
| 10 | Error Handling | `Option`, `Result`, `?`, custom errors |

## How to use this repository

1. Start with lesson 1 and move forward in order.
2. Read `src/main.rs` first to see a complete, runnable example.
3. Open `src/exercises.rs` and implement the TODO items yourself.
4. Compare your work with `src/solutions.rs` after you try the exercises.

## Running a lesson

From the repository root, enter a lesson directory and run Cargo:

```bash
cd lesson_01_hello_world && cargo run
```

Every lesson is also part of the workspace, so you can verify everything from the root with:

```bash
cargo build --workspace
```

## Working on exercises

- Edit `src/exercises.rs`
- Replace each `todo!("Your implementation here")` with your code
- Re-run `cargo build` or `cargo test` if you add tests of your own
- Compare your approach with `src/solutions.rs`

## Helpful reference

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
