use itertools::Itertools;
use std::fs;

fn main() {
    let count = fs::read_to_string("resources/day2.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|command| (command.0, str::parse(command.1).unwrap()))
        .fold(
            (0, 0),
            |(horizontal_depth, vertical_depth), (direction, magnitude): (&str, i32)| {
                match direction {
                    "forward" => (horizontal_depth + magnitude, vertical_depth),
                    "up" => (horizontal_depth, vertical_depth - magnitude),
                    "down" => (horizontal_depth, vertical_depth + magnitude),
                    _ => panic!("Unexpected direction"),
                }
            },
        );

    println!("{:?}", count);
    println!("{}", count.0 * count.1);
}
