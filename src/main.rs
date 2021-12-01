use itertools::Itertools;
use std::fs;

fn main() {
    let count = fs::read_to_string("resources/day1.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows()
        .filter(|(first, second)| second > first)
        .count();

    println!("{}", count);
}
