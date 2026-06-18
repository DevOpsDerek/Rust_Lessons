#![allow(dead_code)]

/// SOLUTION: Capture `bonus` inside a closure and map over the scores.
pub fn exercise_1(scores: Vec<i32>, bonus: i32) -> Vec<i32> {
    let add_bonus = |score: i32| score + bonus;
    scores.into_iter().map(add_bonus).collect()
}

/// SOLUTION: Filter, map, and collect in one iterator chain.
pub fn exercise_2(values: Vec<i32>) -> Vec<i32> {
    values
        .into_iter()
        .filter(|value| value % 2 == 0)
        .map(|value| value * 2)
        .collect()
}

/// SOLUTION: `fold` accumulates the running total.
pub fn exercise_3(words: Vec<&str>) -> usize {
    words.into_iter().fold(0, |acc, word| acc + word.len())
}
