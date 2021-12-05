use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::{fs, str::FromStr};

#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("{x},{y}")]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("{start} -> {end}")]
struct Vector {
    pub start: Point,
    pub end: Point,
}

impl Vector {
    pub fn points_touched(&self) -> Vec<(i32, i32)> {
        let rx = if self.start.x < self.end.x {
            self.start.x..=self.end.x
        } else {
            self.end.x..=self.start.x
        };

        let ry = if self.start.y < self.end.y {
            self.start.y..=self.end.y
        } else {
            self.end.y..=self.start.y
        };

        rx.cartesian_product(ry).collect()
    }
}

fn main() {
    let input = fs::read_to_string("resources/day5.txt")
        .unwrap()
        .split("\n")
        .filter_map(|str| Vector::from_str(str).ok())
        .filter(|vec| vec.start.x == vec.end.x || vec.start.y == vec.end.y)
        .collect::<Vec<Vector>>();

    let points = input
        .into_iter()
        .map(|vector| vector.points_touched())
        .flatten()
        .counts()
        .into_iter()
        .filter(|&(_, freq)| freq > 1)
        .count();

    println!("{:?}", points);
}
