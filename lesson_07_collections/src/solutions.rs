#![allow(dead_code)]

use std::collections::HashMap;

/// SOLUTION: Create a vector and sum its contents.
pub fn exercise_1(a: i32, b: i32, c: i32) -> i32 {
    [a, b, c].iter().sum()
}

/// SOLUTION: Use `iter_mut()` to change each element in place.
pub fn exercise_2(values: &mut [i32]) {
    for value in values.iter_mut() {
        *value += 5;
    }
}

/// SOLUTION: Consume the pairs and insert them into a new hash map.
pub fn exercise_3(pairs: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for (name, score) in pairs {
        map.insert(name, score);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_sums_three_values() {
        assert_eq!(exercise_1(1, 2, 3), 6);
        assert_eq!(exercise_1(0, 0, 0), 0);
        assert_eq!(exercise_1(-1, 0, 1), 0);
    }

    #[test]
    fn test_exercise_2_adds_five_to_each() {
        let mut v = vec![1, 2, 3];
        exercise_2(&mut v);
        assert_eq!(v, vec![6, 7, 8]);
    }

    #[test]
    fn test_exercise_2_empty_vec() {
        let mut v: Vec<i32> = vec![];
        exercise_2(&mut v);
        assert_eq!(v, vec![]);
    }

    #[test]
    fn test_exercise_3_builds_hashmap() {
        let pairs = vec![("Alice".to_string(), 95), ("Bob".to_string(), 80)];
        let map = exercise_3(pairs);
        assert_eq!(map["Alice"], 95);
        assert_eq!(map["Bob"], 80);
    }
}
