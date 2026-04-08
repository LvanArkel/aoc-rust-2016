use std::iter::once;

use itertools::Itertools;

use crate::day::AocDay;

pub struct Day18;

fn iterate(row: Vec<bool>) -> Vec<bool> {
    let window = 
        once(false)
        .chain(row.into_iter())
        .chain(once(false));
    window.tuple_windows().map(|(a, b, c)| {
        (a && b && !c) ||
        (!a && b && c) ||
        (a && !b && !c) ||
        (!a && !b && c)
    }).collect()
}

fn find_safe_tiles(first_row: &Vec<bool>, rows: usize) -> usize {
    let mut total = 0;
    let mut row = first_row.clone();
    for _ in 0..rows {
        total += row.iter().filter(|v| !(**v)).count();
        row = iterate(row);
    }
    total
}

impl AocDay for Day18 {
    type I = Vec<bool>;

    type O = usize;

    fn filename() -> &'static str {
        "input/day18.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.chars().map(|c| c == '^').collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        find_safe_tiles(input, 40)
    }

    fn part2(input: &Self::I) -> Self::O {
        find_safe_tiles(input, 400000)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day18::{Day18, find_safe_tiles}};

    #[test]
    fn test_part1() {
        assert_eq!(6, find_safe_tiles(&Day18::parse("..^^."), 3));
        assert_eq!(38, find_safe_tiles(&Day18::parse(".^^.^.^^^^"), 10));
    }
}