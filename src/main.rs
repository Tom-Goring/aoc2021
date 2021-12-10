use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("./resources/day7.txt")
        .expect("File not found")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let min_fuel = (0..*input.iter().max().unwrap()).fold(i32::MAX, |min, end| {
        cmp::min(
            min,
            input.iter().fold(0, |fuel, start| {
                fuel + ((end - start).abs().pow(2) + (end - start).abs()) / 2
            }),
        )
    });

    println!("{}", min_fuel);
}
