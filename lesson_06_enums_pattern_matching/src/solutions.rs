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

/// SOLUTION: Match each traffic light variant explicitly.
pub fn exercise_1(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Prepare to stop",
        TrafficLight::Green => "Drive forward",
    }
}

/// SOLUTION: Return a friendly string based on the enum contents.
pub fn exercise_2(score: Score) -> String {
    match score {
        Score::Points(value) => format!("Score recorded: {value}"),
        Score::Missing => "No score was provided".to_string(),
    }
}

/// SOLUTION: `while let` keeps looping while `pop()` returns `Some`.
pub fn exercise_3(mut values: Vec<i32>) -> i32 {
    let mut total = 0;
    while let Some(value) = values.pop() {
        total += value;
    }
    total
}
