#![allow(dead_code)]
#![allow(clippy::ptr_arg)] // push() requires &mut String, not &mut str

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_len_of_moved_string() {
        assert_eq!(exercise_1(String::from("hello")), 5);
        assert_eq!(exercise_1(String::new()), 0);
    }

    #[test]
    fn test_exercise_2_first_word() {
        assert_eq!(exercise_2("hello world"), "hello");
        assert_eq!(exercise_2("rust"), "rust");
        assert_eq!(exercise_2("one two three"), "one");
    }

    #[test]
    fn test_exercise_2_empty_string() {
        assert_eq!(exercise_2(""), "");
    }

    #[test]
    fn test_exercise_3_appends_exclamation() {
        let mut s = String::from("Hello");
        exercise_3(&mut s);
        assert_eq!(s, "Hello!");
    }
}
