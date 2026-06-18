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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise_1_traffic_light_messages() {
        assert_eq!(exercise_1(TrafficLight::Red), "Stop");
        assert_eq!(exercise_1(TrafficLight::Yellow), "Prepare to stop");
        assert_eq!(exercise_1(TrafficLight::Green), "Drive forward");
    }

    #[test]
    fn test_exercise_2_score_with_points() {
        let result = exercise_2(Score::Points(42));
        assert!(result.contains("42"));
    }

    #[test]
    fn test_exercise_2_missing_score() {
        let result = exercise_2(Score::Missing);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_exercise_3_sums_all_values() {
        assert_eq!(exercise_3(vec![1, 2, 3, 4]), 10);
        assert_eq!(exercise_3(vec![]), 0);
        assert_eq!(exercise_3(vec![100]), 100);
    }
}
