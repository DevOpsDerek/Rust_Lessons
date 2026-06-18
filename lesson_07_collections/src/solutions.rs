#![allow(dead_code)]

use std::collections::HashMap;

/// SOLUTION: Create a vector and sum its contents.
pub fn exercise_1(a: i32, b: i32, c: i32) -> i32 {
    let values = vec![a, b, c];
    values.iter().sum()
}

/// SOLUTION: Use `iter_mut()` to change each element in place.
pub fn exercise_2(values: &mut Vec<i32>) {
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
