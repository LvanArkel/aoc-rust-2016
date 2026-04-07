use core::num;
use std::collections::VecDeque;

use indicatif::ProgressBar;

use crate::day::AocDay;

pub struct Day19;

impl AocDay for Day19 {
    type I = usize;

    type O = usize;

    fn filename() -> &'static str {
        "input/day19.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.parse().unwrap()
    }

    fn part1(input: &Self::I) -> Self::O {
        let mut numbers: Vec<_> = (1..=*input).collect();
        let mut keep_index = 0;
        while numbers.len() > 1 {
            let mut should_keep_next = keep_index == (numbers.len() % 2);
            numbers = numbers
                .into_iter()
                .enumerate()
                .filter(|(i, _)| i % 2 == keep_index)
                .map(|(_, n)| n)
                .collect();
            keep_index = if should_keep_next {
                0
            } else {
                1
            }
        }
        numbers[0]
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut numbers: VecDeque<usize> = (1..=*input).collect();
        let progress_bar = ProgressBar::new(*input as u64);
        let mut last_i = 0;
        while numbers.len() > 1 {
            let i = (last_i + numbers.len() / 2) % numbers.len();
            // println!("{numbers:?} [{last_i}] -> {i}");
            numbers.remove(i);
            progress_bar.inc(1);
            last_i = if last_i == numbers.len() {
                0
            } else if i < last_i {
                last_i
            } else {
                last_i + 1
            };
        }
        progress_bar.finish();
        numbers[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day19::Day19};

    #[test]
    fn test_part1() {
        assert_eq!(3, Day19::part1(&5));
        assert_eq!(7, Day19::part1(&7));
        assert_eq!(3, Day19::part1(&9));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, Day19::part2(&5));
        assert_eq!(5, Day19::part2(&7));
        assert_eq!(9, Day19::part2(&9));
    }
}