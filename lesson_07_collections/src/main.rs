#![allow(unused_variables)]

mod exercises;
mod solutions;

use std::collections::HashMap;

fn main() {
    println!("Lesson 07: Collections: Vec & HashMap");
    println!("--------------------------------------");

    // LEARN: vec! is shorthand for Vec::new() + push calls — both produce the same Vec.
    let mut numbers = vec![10, 20, 30];
    println!("Vector after pushes: {:?}", numbers);

    let first = numbers[0];
    println!("First element by index: {first}");

    for value in numbers.iter() {
        println!("Borrowed with iter(): {value}");
    }

    for value in numbers.iter_mut() {
        *value += 1;
    }
    println!("After iter_mut bonus: {:?}", numbers);

    let consumed_total: i32 = numbers.into_iter().sum();
    println!("Total after into_iter consumes vector: {consumed_total}");

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Ava"), 95);
    scores.insert(String::from("Noah"), 88);
    scores
        .entry(String::from("Ava"))
        .and_modify(|score| *score += 1);

    for (name, score) in &scores {
        println!("{name} => {score}");
    }

    if let Some(score) = scores.get("Ava") {
        println!("Ava's score is {score}");
    }
}
