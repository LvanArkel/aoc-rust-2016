use std::ops::{RangeBounds, RangeInclusive};

use crate::day::AocDay;

pub struct Day20;

fn segment_without(segment: (u32, u32), range: &(u32, u32)) -> Vec<(u32, u32)> {
    // Cases:
    if range.1 < segment.0 {
        // Range is before segment
        vec![segment]
    } else if range.0 > segment.1 {
        // Range is after segment
        vec![segment]
    } else if range.0 <= segment.0 && range.1 >= segment.1 {
        // Range wraps around segment
        vec![]
    } else if range.0 <= segment.0 && range.1 < segment.1 {
        // Range contains start of segment
        vec![(range.1 + 1, segment.1)]
    } else if range.0 > segment.0 && range.1 >= segment.1 {
        // Range contains end of segment
        vec![(segment.0, range.0 - 1)]
    } else {
        // Range is within segment
        vec![(segment.0, range.0 - 1), (range.1 + 1, segment.1)]
    }
}

impl AocDay for Day20 {
    type I = Vec<(u32, u32)>;

    type O = u32;

    fn filename() -> &'static str {
        "input/day20.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.lines().map(|line| {
            let (a, b) = line.split_once("-").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        }).collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        input.iter().fold(vec![(0_u32, u32::MAX)], |segments, range| {
            segments
                .into_iter()
                .flat_map(|segment| segment_without(segment, &range))
                .collect()
        }).first().unwrap().0
    }

    fn part2(input: &Self::I) -> Self::O {
        input.iter().fold(vec![(0_u32, u32::MAX)], |segments, range| {
            segments
                .into_iter()
                .flat_map(|segment| segment_without(segment, &range))
                .collect()
        }).iter().map(|segment| {
            segment.1 - segment.0 + 1
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day20::Day20};

    #[test]
    fn test_part1() {
        let ranges = Day20::parse(r#"5-8
0-2
4-7"#);
        assert_eq!(3, Day20::part1(&ranges))
    }
}
