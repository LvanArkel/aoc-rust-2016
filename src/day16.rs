use std::iter::once;

use itertools::Itertools;

use crate::day::AocDay;

pub struct Day16;

fn expand_step(line: Vec<bool>) -> Vec<bool> {
    let a = line.iter().map(|v| *v);
    let b = line.iter().rev().map(|v| !(*v));
    let mid = once(false);

    a.chain(mid).chain(b).collect()
}

fn generate_length(init: Vec<bool>, size: usize) -> Vec<bool> {
    let mut accumulator = init;
    while accumulator.len() < size {
        accumulator = expand_step(accumulator);
    }
    accumulator
}

fn generate_checksum(sequence: Vec<bool>) -> Vec<bool> {
    let mut accumulator = sequence;
    while accumulator.len() % 2 == 0 {
        accumulator = accumulator.chunks(2).map(|x| {
            x[0] == x[1]
        }).collect();
    }
    accumulator
}

fn to_int(checksum: Vec<bool>) -> usize {
    checksum.iter().fold(0, |acc, b| {
        acc * 2 + if *b { 1 } else { 0 }
    })
}

fn to_value(checksum: Vec<bool>) -> String {
    checksum.into_iter().map(|c| {
        if c {'1'} else {'0'}
    }).collect()
}

fn create_checksum(init: Vec<bool>, size: usize) -> Vec<bool> {
    let expanded = generate_length(init, size);
    generate_checksum(expanded.into_iter().take(size).collect())
}

impl AocDay for Day16 {
    type I = Vec<bool>;

    type O = String;

    fn filename() -> &'static str {
        "input/day16.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.chars().map(|c| c == '1').collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        let size = 272;
        let checksum = create_checksum(input.clone(), size);
        to_value(checksum)
    }

    fn part2(input: &Self::I) -> Self::O {
        let size = 35651584;
        let checksum = create_checksum(input.clone(), size);
        to_value(checksum)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day16::{Day16, create_checksum, expand_step, generate_checksum}};

    #[test]
    fn test_expand() {
        let test_data = vec![
            ("1", "100"),
            ("0", "001"),
            ("11111", "11111000000"),
            ("111100001010", "1111000010100101011110000"),
        ];
        for (start, end) in test_data {
            let start = Day16::parse(start);
            let end = Day16::parse(end);
            assert_eq!(end, expand_step(start));
        }
    }

    #[test]
    fn test_checksum() {
        let input = Day16::parse("110010110100");
        assert_eq!(Day16::parse("100"), generate_checksum(input));
    }

    #[test]
    fn test_whole() {
        let input = Day16::parse("10000");
        let size = 20;
        assert_eq!(Day16::parse("01100"), create_checksum(input, size));
    }
}