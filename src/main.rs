use itertools::Itertools;
use std::fs;

fn main() {
    let count = fs::read_to_string("resources/day2.test")
        .unwrap()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap()
        })
        .map(|command| (command.0, str::parse(command.1).unwrap()))
        .fold(
            (0, 0, 0),
            |(horizontal_position, vertical_depth, aim), (direction, magnitude): (&str, i32)| {
                println!("{} {}", direction, magnitude);
                match direction {
                    "forward" => (
                        horizontal_position + magnitude,
                        vertical_depth + aim * magnitude,
                        aim,
                    ),
                    "up" => (horizontal_position, vertical_depth, aim - magnitude),
                    "down" => (horizontal_position, vertical_depth, aim + magnitude),
                    _ => panic!("Unexpected direction"),
                }
            },
        );

    println!("{:?}", count);
    println!("{}", count.0 * count.1);
}
