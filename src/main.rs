use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("./resources/day7.txt")
        .expect("File not found")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .sorted()
        .collect::<Vec<i32>>();

    let median = input[input.len() / 2];

    let fuel = input
        .iter()
        .sorted()
        .fold(0, |fuel, pos| fuel + (pos - median).abs());

    println!("{}", fuel);
}
