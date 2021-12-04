use std::fs;

#[derive(Debug)]
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

    let boards = input
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

    for idx in 1..=numbers.len() {
        println!("Testing with: {:?} (idx: {})", &numbers[0..idx + 1], idx);
        if let Some(board) = boards.iter().find(|b| b.check_for_bingo(&numbers[0..idx])) {
            println!("Board found on idx: {} - {:?}", idx, board);
            for row in board.rows.iter() {
                println!("{:?}", row);
            }
            let sum = board.unmarked_numbers(&numbers[0..idx]).iter().sum::<i32>();
            println!(
                "{} * {} = {}",
                sum,
                &numbers[idx - 1],
                sum * &numbers[idx - 1]
            );
            break;
        }
    }
}
