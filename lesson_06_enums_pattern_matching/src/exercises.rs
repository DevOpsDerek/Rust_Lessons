#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Score {
    Points(u32),
    Missing,
}

/// Return the suggested action for a `TrafficLight` enum value.
pub fn exercise_1(light: TrafficLight) -> &'static str {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Match on `Score::Points(u32)` or `Score::Missing` and return a message `String`.
pub fn exercise_2(score: Score) -> String {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}

/// Use `while let` to pop all numbers from a vector and return their sum.
pub fn exercise_3(values: Vec<i32>) -> i32 {
    // TODO: Replace this placeholder with your own implementation.
    todo!("Your implementation here")
}
