#![allow(unused_variables)]

mod exercises;
mod solutions;

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Lesson 06: Enums & Pattern Matching");
    println!("------------------------------------");

    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: 4 },
        Message::Write(String::from("hello match")),
        Message::ChangeColor(255, 128, 0),
    ];
    for message in messages {
        describe_message(message);
    }

    let optional_name = Some("Ferris");
    if let Some(name) = optional_name {
        println!("if let found a name: {name}");
    }

    let mut stack = vec![1, 2, 3];
    while let Some(value) = stack.pop() {
        println!("while let popped {value}");
    }

    for traffic in [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green] {
        let action = light_action(traffic);
        println!("Traffic light action: {action}");
    }
}

#[derive(Debug, Clone, Copy)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn describe_message(message: Message) {
    match message {
        Message::Quit => println!("Received Quit"),
        Message::Move { x, y } => println!("Move to ({x}, {y})"),
        Message::Write(text) => println!("Write text: {text}"),
        Message::ChangeColor(r, g, b) => println!("Change color to ({r}, {g}, {b})"),
    }
}

fn light_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Slow down",
        TrafficLight::Green => "Go",
    }
}
