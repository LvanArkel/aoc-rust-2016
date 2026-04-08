use core::panic;
use std::{collections::VecDeque, iter::once, sync::LazyLock};

use itertools::Itertools;
use regex::Regex;

use crate::day::AocDay;

pub struct Day21;

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    SwapPos{ a: usize, b: usize },
    SwapChar{ a: char, b: char },
    RotateL{ n: usize },
    RotateR{ n: usize },
    RotatePos{ c: char },
    Reverse{ a: usize, b: usize },
    Move{ a: usize, b: usize },
}

fn scramble(text: String, operations: &Vec<Operation>) -> Option<String> {
    let mut chars: VecDeque<_> = text.chars().collect();
    for operation in operations {
        match operation {
            Operation::SwapPos { a, b } => {
                chars.swap(*a, *b);
            },
            Operation::SwapChar { a, b } => {
                for c in chars.iter_mut() {
                    if c == a {
                        *c = *b;
                    } else if c == b {
                        *c = *a;
                    }
                }
            },
            Operation::RotateL { n } => {
                chars.rotate_left(*n);
            },
            Operation::RotateR { n } => {
                chars.rotate_right(*n);
            }
            Operation::RotatePos { c } => {
                let mut i = chars.iter().position(|v| v == c)? + 1;
                if i >= 5 {
                    i += 1;
                };
                chars.rotate_right(i % chars.len());
            },
            Operation::Reverse { a, b } => {
                let mut start = *a;
                let mut end = *b;
                while start < end {
                    chars.swap(start, end);
                    start += 1;
                    end -= 1;
                }
            },
            Operation::Move { a, b } => {
                let x = chars.remove(*a)?;
                chars.insert(*b, x);
            },
        }
    }
    Some(chars.iter().collect())
}

fn generate_sequences(chars: &Vec<char>, size: usize) -> Vec<Vec<char>> {
    if size == 1 {
        return chars.iter().map(|c| vec![*c]).collect()
    }
    generate_sequences(chars, size - 1 ).iter().flat_map(|subseq| {
        chars.iter().map(|c| {
            subseq.iter().map(|c| *c).chain(once(*c)).collect()
        })
    }).collect()
}

fn unscramble(text: String, operations: &Vec<Operation>) -> String {
    let chars = text.chars().unique().collect();

    generate_sequences(&chars, text.len()).into_iter().find(|seq| {
        let unscrambled: String = seq.iter().collect();
        if let Some(scrambled) = scramble(unscrambled, operations) {
            scrambled == text
        } else {
            false
        }
    }).unwrap().iter().collect()

    // let rotate_cache: HashMap<usize, usize> = (0..text.len()).map(|i| {
    //     let j = if i >= 4 {
    //         2*i + 2
    //     } else {
    //         2*i + 1
    //     } % text.len();
    //     println!("{i} -> {j}");
    //     (j, i)
    // }).collect();

    // let mut chars: VecDeque<_> = text.chars().collect();
    // for operation in operations.iter().rev() {
    //     println!("{chars:?} {operation:?}");
    //     match operation {
    //         Operation::SwapPos { a, b } => {
    //             chars.swap(*a, *b);
    //         },
    //         Operation::SwapChar { a, b } => {
    //             for c in chars.iter_mut() {
    //                 if c == a {
    //                     *c = *b;
    //                 } else if c == b {
    //                     *c = *a;
    //                 }
    //             }
    //         },
    //         Operation::RotateL { n } => {
    //             chars.rotate_right(*n);
    //         },
    //         Operation::RotateR { n } => {
    //             chars.rotate_left(*n);
    //         },
    //         Operation::RotatePos { c } => {
    //             let m = chars.iter().position(|v| v == c).unwrap();
    //             let i = rotate_cache.get(&m).unwrap();
    //             chars.rotate_left(m - i);
    //         },
    //         Operation::Reverse { a, b } => {
    //             let mut start = *a;
    //             let mut end = *b;
    //             while start < end {
    //                 chars.swap(start, end);
    //                 start += 1;
    //                 end -= 1;
    //             }
    //         },
    //         Operation::Move { a, b } => {
    //             let x = chars.remove(*b).unwrap();
    //             chars.insert(*a, x);
    //         },
    //     }
    // }
    // chars.iter().collect()
}

static SWAP_POS_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"swap position (\d+) with position (\d+)"#
).unwrap());
static SWAP_CHAR_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"swap letter (\w) with letter (\w)"#
).unwrap());
static ROTATE_L_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"rotate left (\d+)"#
).unwrap());
static ROTATE_R_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"rotate right (\d+)"#
).unwrap());
static ROTATE_POS_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"rotate based on position of letter (\w)"#
).unwrap());
static REVERSE_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"reverse positions (\d+) through (\d+)"#
).unwrap());
static MOVE_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"move position (\d+) to position (\d+)"#
).unwrap());

impl AocDay for Day21 {
    type I = Vec<Operation>;

    type O = String;

    fn filename() -> &'static str {
        "input/day21.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.lines().map(|line| {
            if let Some(cap) = SWAP_POS_PATTERN.captures(line) {
                let a = cap.get(1).unwrap().as_str().parse().unwrap();
                let b = cap.get(2).unwrap().as_str().parse().unwrap();
                Operation::SwapPos { a, b }
            } else if let Some(cap) = SWAP_CHAR_PATTERN.captures(line) {
                let a = cap.get(1).unwrap().as_str().chars().nth(0).unwrap();
                let b = cap.get(2).unwrap().as_str().chars().nth(0).unwrap();
                Operation::SwapChar { a, b }
            } else if let Some(cap) = ROTATE_L_PATTERN.captures(line) {
                let n = cap.get(1).unwrap().as_str().parse().unwrap();
                Operation::RotateL { n }
            } else if let Some(cap) = ROTATE_R_PATTERN.captures(line) {
                let n = cap.get(1).unwrap().as_str().parse().unwrap();
                Operation::RotateR { n }
            } else if let Some(cap) = ROTATE_POS_PATTERN.captures(line) {
                let c = cap.get(1).unwrap().as_str().chars().nth(0).unwrap();
                Operation::RotatePos { c }
            } else if let Some(cap) = REVERSE_PATTERN.captures(line) {
                let a = cap.get(1).unwrap().as_str().parse().unwrap();
                let b = cap.get(2).unwrap().as_str().parse().unwrap();
                Operation::Reverse { a, b }
            } else if let Some(cap) = MOVE_PATTERN.captures(line) {
                let a = cap.get(1).unwrap().as_str().parse().unwrap();
                let b = cap.get(2).unwrap().as_str().parse().unwrap();
                Operation::Move { a, b }
            } else {
                panic!("Could not parse line {line}")
            }
        }).collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        scramble(String::from("abcdefgh"), input).unwrap()
    }

    fn part2(input: &Self::I) -> Self::O {
        unscramble(String::from("fbgdceah"), input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day21::{Day21, Operation, scramble, unscramble}};

    #[test]
    fn test_part1() {
        let operations = Day21::parse(r#"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d"#);
        assert_eq!(Some("decab".to_string()), scramble("abcde".to_string(), &operations));
    }

    #[test]
    fn test_part2() {
        let operations = Day21::parse(r#"swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d"#);
        assert_eq!("abcde".to_string(), unscramble("decab".to_string(), &operations));
        assert_eq!("abcdefgh".to_string(), unscramble("bgfacdeh".to_string(), &operations))
    }

    #[test]
    fn test_unscramble_rotate_pos() {
        let text = "abcde";

        for char in text.chars() {
            let ops = vec![Operation::RotatePos { c: char }];
            let scrambled = scramble(text.to_string(), &ops).unwrap();
            println!("Scrambled {} => {}", text, scrambled);
            assert_eq!("abcde", unscramble(scrambled, &ops), "Unscrambling {}", char);
        }
    }
}