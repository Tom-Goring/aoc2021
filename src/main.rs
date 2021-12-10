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
        // println!("Getting points of {}", &self);
        let rx: Vec<i32> = if self.start.x < self.end.x {
            (self.start.x..=self.end.x).collect()
        } else {
            (self.end.x..=self.start.x).rev().collect()
        };

        let ry: Vec<i32> = if self.start.y < self.end.y {
            (self.start.y..=self.end.y).collect()
        } else {
            (self.end.y..=self.start.y).rev().collect()
        };

        if (self.end.x - self.start.x).abs() == (self.end.y - self.start.y).abs() {
            rx.into_iter().zip(ry).collect()
        } else {
            rx.into_iter().cartesian_product(ry).collect()
        }
    }
}

fn main() {
    let input = fs::read_to_string("resources/day5.txt")
        .unwrap()
        .split("\n")
        .filter_map(|str| Vector::from_str(str).ok())
        // .inspect(|vec| println!("{}", vec))
        .collect::<Vec<Vector>>();

    let points = input
        .clone()
        .into_iter()
        .map(|vector| vector.points_touched())
        .flatten()
        // .inspect(|point| println!("{:?}", point))
        .counts()
        .into_iter()
        .filter(|&(_, freq)| freq > 1)
        .count();

    println!("{}", points);

    // for y in 0..input.len() as i32 {
    //     for x in 0..input.len() as i32 {
    //         if let Some(count) = points.get(&(x, y)) {
    //             print!("{} ", count);
    //         } else {
    //             print!(". ");
    //         }
    //     }
    //     println!();
    // }
}
