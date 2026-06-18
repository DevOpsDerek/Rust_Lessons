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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_adds_bonus_to_each() {
        assert_eq!(exercise_1(vec![10, 20, 30], 5), vec![15, 25, 35]);
        assert_eq!(exercise_1(vec![], 5), vec![]);
    }

    #[test]
    fn test_exercise_1_zero_bonus() {
        assert_eq!(exercise_1(vec![1, 2, 3], 0), vec![1, 2, 3]);
    }

    #[test]
    fn test_exercise_2_filters_evens_and_doubles() {
        assert_eq!(exercise_2(vec![1, 2, 3, 4, 5, 6]), vec![4, 8, 12]);
        assert_eq!(exercise_2(vec![1, 3, 5]), vec![]);
    }

    #[test]
    fn test_exercise_3_total_char_count() {
        assert_eq!(exercise_3(vec!["hi", "there"]), 7);
        assert_eq!(exercise_3(vec![]), 0);
        assert_eq!(exercise_3(vec!["rust"]), 4);
    }
}
