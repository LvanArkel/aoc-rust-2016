use std::collections::HashSet;

use crate::AocDay;
use crate::utils::direction::Direction;

pub struct Day1;

impl AocDay for Day1 {
    type I = Vec<(char, i32)>;
    type O = i32;

    fn filename(&self) -> &'static str {
        "input/day1.txt"
    }

    fn parse(&self, contents: &str) -> Vec<(char, i32)> {
        contents.split(", ").map(|segment| {
            let c = segment.chars().nth(0).unwrap();
            println!("{} {}", c, &segment[1..]);
            let digit = segment[1..].parse().unwrap();
            (c, digit)
        }).collect()
    }

    fn part1(&self, input: &Vec<(char, i32)>) -> i32 {
        let (_, position) = input.iter().fold((Direction::North, (0, 0)), |(facing, (x, y)), direction| {
            let new_facing = match direction.0 {
                'L' => facing.left(),
                'R' => facing.right(),
                _ => panic!("Unknown direction {}", direction.0)
            };
            let offset = new_facing.offset();
            (new_facing, (x + offset.0 * direction.1, y + offset.1 * direction.1))
        });
        position.0.abs() + position.1.abs()
    }

    fn part2(&self, input: &Vec<(char, i32)>) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let (mut x, mut y) = (0, 0);
        let mut direction = Direction::North;
        for (facing, amount) in input {
            direction = match facing {
                'L' => direction.left(),
                'R' => direction.right(),
                _ => panic!("Unknown direction {}", facing)
            };
            let offset = direction.offset();
            for _ in 0..*amount {
                x += offset.0;
                y += offset.1;
                if visited.contains(&(x, y)) {
                    return x.abs() + y.abs();
                }
                visited.insert((x, y));
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{AocDay, day1::Day1};

    #[test]
    fn test_part1() {
        let day1 = Day1;
        assert_eq!(5, day1.part1(&day1.parse("R2, L3")));
        assert_eq!(2, day1.part1(&day1.parse("R2, R2, R2")));
        assert_eq!(12, day1.part1(&day1.parse("R5, L5, R5, R3")));
    }
}