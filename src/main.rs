use std::fs;

#[derive(Debug, Clone)]
struct Board {
    rows: Vec<Vec<i32>>,
    cols: Vec<Vec<i32>>,
}

impl Board {
    pub fn new(rows: Vec<Vec<i32>>) -> Self {
        let cols: Vec<Vec<i32>> = (0..rows.len())
            .map(|i| rows.iter().map(|c| c[i]).collect())
            .collect();
        Board { rows, cols }
    }

    pub fn check_for_bingo(&self, winning_numbers: &[i32]) -> bool {
        self.rows
            .iter()
            .any(|row| row.iter().all(|elem| winning_numbers.contains(elem)))
            || self
                .cols
                .iter()
                .any(|col| col.iter().all(|elem| winning_numbers.contains(elem)))
    }

    pub fn unmarked_numbers(&self, winning_numbers: &[i32]) -> Vec<i32> {
        self.rows
            .iter()
            .flatten()
            .copied()
            .filter(|elem| !winning_numbers.contains(elem))
            .collect()
    }
}

use std::fmt;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            write!(f, "{:?}\n", row).unwrap();
        }
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("resources/day4.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let numbers: Vec<i32> = input[0]
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = input
        .iter()
        .skip(1)
        .map(|sg| {
            Board::new(
                sg.split('\n')
                    .map(|row| {
                        row.split_whitespace()
                            .map(|c| c.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>(),
            )
        })
        .collect::<Vec<Board>>();

    println!("{}", numbers.len());

    let mut winning_boards = Vec::new();

    for idx in 1..=numbers.len() {
        boards
            .iter()
            .filter(|b| b.check_for_bingo(&numbers[0..idx]))
            .for_each(|b| {
                println!("Board found on idx: {}", idx);
                winning_boards.push((idx, b.clone()))
            });
        boards = boards
            .iter()
            .cloned()
            .filter(|b| !b.check_for_bingo(&numbers[0..idx]))
            .collect();
    }

    for (_, board) in winning_boards.iter() {
        println!("{}", board);
    }

    if let Some((idx, board)) = winning_boards.last() {
        let sum = board
            .unmarked_numbers(&numbers[0..*idx])
            .iter()
            .sum::<i32>();
        println!("{}", sum * numbers[idx - 1]);
    }
}
