use std::sync::LazyLock;

use regex::Regex;

use crate::day::AocDay;

pub struct Day9;

static pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\((\d+)x(\d+)\)"#).unwrap());

fn parse_pattern(input: &str) -> (usize, usize, usize) {
    let capture = pattern.captures(input).unwrap();
    let m = capture.get_match();
    let size: usize = capture.get(1).unwrap().as_str().parse().unwrap();
    let amount = capture.get(2).unwrap().as_str().parse().unwrap();
    let start = m.end();
    let end = start + size;
    (amount, start, end)
}

fn part2_recursive(input: &str) -> usize {
    match input.find('(') {
        None => input.len(),
        Some(i) => {
            let rest = &input[i..];
            let (amount, start, end) = parse_pattern(rest);
            i +
            amount * part2_recursive(&rest[start..end]) +
            part2_recursive(&rest[end..])
        },
    }
}

impl AocDay for Day9 {
    type I = String;

    type O = usize;

    fn filename(&self) -> &'static str {
        "input/day9.txt"
    }

    fn parse(&self, contents: &str) -> Self::I {
        contents.to_owned()
    }

    fn part1(&self, input: &Self::I) -> Self::O {
        let mut head = 0;
        let mut total = 0;
        for capture in pattern.captures_iter(input) {
            let m = capture.get_match();
            let start = m.start();
            if start < head { continue; }
            total += start - head;
            let size: usize = capture.get(1).unwrap().as_str().parse().unwrap();
            let amount: usize = capture.get(2).unwrap().as_str().parse().unwrap();
            total += size * amount;
            head = m.end() + size;
        }
        let rest = (input.len() - head).max(0);
        total + rest
    }

    fn part2(&self, input: &Self::I) -> Self::O {
        part2_recursive(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day9::Day9};

    #[test]
    fn test_part1() {
        let day = Day9;
        assert_eq!(6, day.part1(&"ADVENT".to_owned()));
        assert_eq!(7, day.part1(&"A(1x5)BC".to_owned()));
        assert_eq!(9, day.part1(&"(3x3)XYZ".to_owned()));
        assert_eq!(11, day.part1(&"A(2x2)BCD(2x2)EFG".to_owned()));
        assert_eq!(6, day.part1(&"(6x1)(1x3)A".to_owned()));
        assert_eq!(18, day.part1(&"X(8x2)(3x3)ABCY".to_owned()));
    }

    #[test]
    fn test_part2() {
        let day = Day9;
        assert_eq!(9, day.part2(&"(3x3)XYZ".to_owned()));
        assert_eq!(20, day.part2(&"X(8x2)(3x3)ABCY".to_owned()));
        assert_eq!(241920, day.part2(&"(27x12)(20x12)(13x14)(7x10)(1x12)A".to_owned()));
        assert_eq!(445, day.part2(&"(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_owned()));
    }
}
