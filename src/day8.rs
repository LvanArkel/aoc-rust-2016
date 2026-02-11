use std::sync::LazyLock;

use fancy_regex::Regex;
use itertools::Itertools;

use crate::day::AocDay;

pub enum Operation {
    Rect{ rows: usize, cols: usize },
    RotateRow { row: usize, amount: usize },
    RotateCol { col: usize, amount: usize },
}

static rect_pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"rect (\d+)x(\d+)"#).unwrap());
static rotate_row_pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"rotate row y=(\d+) by (\d+)"#).unwrap());
static rotate_column_pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"rotate column x=(\d+) by (\d+)"#).unwrap());

impl Operation {
    fn parse(input: &str) -> Result<Operation, String> {
        if let Some(cap) = rect_pattern.captures(input).unwrap() {
            let cols = cap.get(1).unwrap().as_str().parse().unwrap();
            let rows = cap.get(2).unwrap().as_str().parse().unwrap();
            return Ok(Operation::Rect { rows, cols })
        }
        if let Some(cap) = rotate_row_pattern.captures(input).unwrap() {
            let row = cap.get(1).unwrap().as_str().parse().unwrap();
            let amount = cap.get(2).unwrap().as_str().parse().unwrap();
            return Ok(Operation::RotateRow { row, amount })
        }
        if let Some(cap) = rotate_column_pattern.captures(input).unwrap() {
            let col = cap.get(1).unwrap().as_str().parse().unwrap();
            let amount = cap.get(2).unwrap().as_str().parse().unwrap();
            return Ok(Operation::RotateCol { col, amount })
        }
        return Err(input.to_owned())
    }    
}

const WIDTH: usize = 50;
const HEIGHT: usize = 6;


pub struct Day8;

fn run_instructions(input: &<Day8 as AocDay>::I) -> [[bool; 50]; 6] {
    let mut grid = [[false; WIDTH]; HEIGHT];
    for operation in input {
        match operation {
            Operation::Rect { rows, cols } => {
                for r in 0..*rows {
                    for c in 0..*cols {
                        grid[r][c] = true;
                    }
                }
            },
            Operation::RotateRow { row, amount } => {
                let mut new_row = [false; WIDTH];
                for i in 0..WIDTH {
                    new_row[(i + amount) % WIDTH] = grid[*row][i];
                }
                for i in 0..WIDTH {
                    grid[*row][i] = new_row[i];
                }
            },
            Operation::RotateCol { col, amount } => {
                let mut new_col = [false; HEIGHT];
                for i in 0..HEIGHT {
                    new_col[(i + amount) % HEIGHT] = grid[i][*col];
                }
                for i in 0..HEIGHT {
                    grid[i][*col] = new_col[i];
                }
            },
        }
    }
    grid
}

impl AocDay for Day8 {
    type I = Vec<Operation>;

    type O = usize;

    fn filename(&self) -> &'static str {
        "input/day8.txt"
    }

    fn parse(&self, contents: &str) -> Self::I {
        contents.lines().map(|line| {
            Operation::parse(line).unwrap()
        }).collect()
    }

    fn part1(&self, input: &Self::I) -> Self::O {
        let grid = run_instructions(input);
        grid.into_iter()
            .map(|row| {
                row.into_iter().filter(|cell| *cell).count()
            })
            .sum()
    }

    fn part2(&self, input: &Self::I) -> Self::O {
        let grid = run_instructions(input);
        for row in grid {
            let line: String = row.iter().map(|c| if *c {'#'} else {'.'}).collect();
            println!("{line}");
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day8::Day8};
}