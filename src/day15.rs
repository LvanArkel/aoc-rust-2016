use std::{sync::LazyLock, usize};

use itertools::Itertools;
use regex::Regex;

use crate::day::AocDay;

pub struct Day15;

#[derive(Clone, Copy)]
pub struct Disc {
    count: usize,
    start: usize,
}

static DISC_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"Disc .+ has (\d+) positions; at time=0, it is at position (\d+)."#
).unwrap());

impl AocDay for Day15 {
    type I = Vec<Disc>;

    type O = usize;

    fn filename() -> &'static str {
        "input/day15.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.lines().map(|line| {
            let cap = DISC_PATTERN.captures(line).unwrap();
            Disc {
                count: cap.get(1).unwrap().as_str().parse().unwrap(),
                start: cap.get(2).unwrap().as_str().parse().unwrap(),
            }
        }).collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        let discs: Vec<_> = input.iter().enumerate().map(|(i, disc)| {
            (disc.count, (disc.start + i + 1) % disc.count)
        }).sorted_by_key(|(c, _)| usize::MAX - c).collect();
        let (first_count, first_pos) = &discs[0];
        let mut time = (first_count - first_pos) % first_count;
        loop {
            let mut mapped_disks = discs.iter().map(|(count, pos)| {
                (pos + time) % count
            });
            if mapped_disks.all(|pos| pos == 0) {
                return time;
            }
            time += first_count;
        }
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut new_discs = input.clone();
        new_discs.push(Disc { count: 11, start: 0 });
        Self::part1(&new_discs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day15::Day15};

    #[test]
    fn test_part1() {
        let parsed = Day15::parse(r#"Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1."#);
        assert_eq!(5, Day15::part1(&parsed));
    }
}