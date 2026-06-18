#![allow(dead_code)]

/// SOLUTION: Use an `if` expression to choose the return value.
pub fn exercise_1(value: i32) -> &'static str {
    if value % 2 == 0 { "even" } else { "odd" }
}

/// SOLUTION: A `loop` can return a value with `break`.
pub fn exercise_2(target: i32) -> i32 {
    let mut current = 0;
    loop {
        if current == target {
            break current;
        }
        current += 1;
    }
}

/// SOLUTION: Iterate over an inclusive range and accumulate the total.
pub fn exercise_3(n: i32) -> i32 {
    let mut total = 0;
    for value in 1..=n {
        total += value;
    }
    total
}
