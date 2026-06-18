#![allow(unused_variables)]

mod exercises;
mod solutions;

fn main() {
    println!("Lesson 09: Closures & Iterators");
    println!("---------------------------------");

    let add_one = |x: i32| x + 1;
    println!("Closure result: {}", add_one(4));

    let bonus = 10;
    // LEARN: This closure captures `bonus` from the surrounding scope.
    let add_bonus = |score: i32| score + bonus;
    println!("Score with bonus: {}", add_bonus(30));

    let mut count = 0;
    {
        // LEARN: This closure mutably captures `count`, so it acts like `FnMut`.
        let mut increment = || {
            count += 1;
            count
        };
        println!("Increment once: {}", increment());
        println!("Increment twice: {}", increment());
    }

    let words = ["rust", "makes", "loops", "expressive"];
    let long_words: Vec<&str> = words
        .iter()
        .copied()
        .filter(|word| word.len() > 4)
        .collect();
    println!("Filtered words: {:?}", long_words);

    let numbers = [1, 2, 3, 4, 5];
    let processed: Vec<i32> = numbers
        .iter()
        .map(|value| value * 2)
        .filter(|value| value > &5)
        .collect();
    println!("Mapped and filtered numbers: {:?}", processed);

    // LEARN: fold is useful when you need custom accumulation (here: building a string).
    let sentence = numbers.iter().fold(String::new(), |mut acc, value| {
        if !acc.is_empty() {
            acc.push_str(", ");
        }
        acc.push_str(&value.to_string());
        acc
    });
    println!("Fold into string: {sentence}");
}
