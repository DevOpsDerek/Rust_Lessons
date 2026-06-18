#![allow(unused_variables)]

mod exercises;
mod solutions;

fn main() {
    println!("Lesson 04: Functions & Control Flow");
    println!("-----------------------------------");

    let sum = add(4, 5);
    println!("4 + 5 = {sum}");

    let number = 7;
    if number % 2 == 0 {
        println!("{number} is even");
    } else {
        println!("{number} is odd");
    }

    let countdown_result = countdown_with_loop(3);
    println!("Loop returned: {countdown_result}");

    let mut while_counter = 0;
    while while_counter < 3 {
        println!("while counter = {while_counter}");
        while_counter += 1;
    }

    for value in 1..=4 {
        if value == 3 {
            println!("Skipping {value} with continue");
            continue;
        }
        println!("for loop value = {value}");
    }

    let total = sum_range(1, 5);
    println!("Sum from 1 to 5 = {total}");
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn countdown_with_loop(start: i32) -> i32 {
    let mut current = start;
    loop {
        println!("loop counter = {current}");
        if current == 0 {
            break current;
        }
        current -= 1;
    }
}

fn sum_range(start: i32, end: i32) -> i32 {
    let mut total = 0;
    for value in start..=end {
        total += value;
    }
    total
}
