use itertools::Itertools;
use std::fs;

fn main() {
    let count = fs::read_to_string("resources/day1.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .tuple_windows::<(i32, i32, i32)>()
        .tuple_windows()
        .filter(|((a, b, c), (x, y, z))| a + b + c < x + y + z)
        .count();

    println!("{}", count);
}
