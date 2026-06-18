#![allow(dead_code)]

/// SOLUTION: Ownership moves into the function, so we can measure the string directly.
pub fn exercise_1(text: String) -> usize {
    text.len()
}

/// SOLUTION: Return the slice up to the first space, or the whole string if none exists.
pub fn exercise_2(text: &str) -> &str {
    for (index, byte) in text.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            return &text[..index];
        }
    }
    text
}

/// SOLUTION: Use a mutable borrow to change the original string in place.
pub fn exercise_3(text: &mut String) {
    text.push('!');
}
