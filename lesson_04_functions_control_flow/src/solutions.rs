#![allow(dead_code)]

/// SOLUTION: Use an `if` expression to choose the return value.
pub fn exercise_1(value: i32) -> &'static str {
    if value % 2 == 0 {
        "even"
    } else {
        "odd"
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_even() {
        assert_eq!(exercise_1(4), "even");
        assert_eq!(exercise_1(0), "even");
    }

    #[test]
    fn test_exercise_1_odd() {
        assert_eq!(exercise_1(7), "odd");
        assert_eq!(exercise_1(-3), "odd");
    }

    #[test]
    fn test_exercise_2_counts_to_target() {
        assert_eq!(exercise_2(0), 0);
        assert_eq!(exercise_2(5), 5);
        assert_eq!(exercise_2(100), 100);
    }

    #[test]
    fn test_exercise_3_sum_1_to_n() {
        assert_eq!(exercise_3(1), 1);
        assert_eq!(exercise_3(5), 15); // 1+2+3+4+5
        assert_eq!(exercise_3(10), 55); // triangular number
        assert_eq!(exercise_3(0), 0);
    }
}
