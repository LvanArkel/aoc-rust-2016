use crate::{day::AocDay, utils::direction::Direction};

pub struct Day2;

impl Day2 {
    fn move_keypad(position: (i32, i32), moves: &Vec<Direction>) ->(i32, i32) {
        moves.iter().fold(position, |acc, direction| {
            let offset = direction.offset();
            let x = (acc.0 + offset.0).clamp(0, 2);
            let y = (acc.1 + offset.1).clamp(0, 2);
            (x, y)
        })
    }

    fn to_digit(position: (i32, i32)) -> i32 {
        match position {
            (0, 0) => 7,
            (1, 0) => 8,
            (2, 0) => 9,
            (0, 1) => 4,
            (1, 1) => 5,
            (2, 1) => 6,
            (0, 2) => 1,
            (1, 2) => 2,
            (2, 2) => 3,
            _ => panic!("Invalid coordinate {position:?}"),
        }
    }
    
    fn move_keypad2(position: (i32, i32), moves: &Vec<Direction>) ->(i32, i32) {
        moves.iter().fold(position, |acc, direction| {
            let offset = direction.offset();
            let x = acc.0 + offset.0 ;
            let y = acc.1 + offset.1 ;
            if ((x-2).abs() + (y-2).abs()) <= 2 {
                (x, y)
            } else {
                acc
            }
        })
    }

    fn to_digit2(position: (i32, i32)) -> u32 {
        match position {
            (2, 0) => 0xD,
            (1, 1) => 0xA,
            (2, 1) => 0xB,
            (3, 1) => 0xC,
            (0, 2) => 0x5,
            (1, 2) => 0x6,
            (2, 2) => 0x7,
            (3, 2) => 0x8,
            (4, 2) => 0x9,
            (1, 3) => 0x2,
            (2, 3) => 0x3,
            (3, 3) => 0x4,
            (2, 4) => 0x1,
            _ => panic!("Unknown value {position:?}")
        }
    }
}

impl AocDay for Day2 {
    type I = Vec<Vec<Direction>>;
    type O = String;
    
    fn filename(&self) -> &'static str {
        "input/day2.txt"
    }
    
    fn parse(&self, contents: &str) -> Self::I {
        contents.lines().map(|line| {
            line.chars().map(|c| {
                match c {
                    'U' => Direction::North,
                    'D' => Direction::South,
                    'L' => Direction::West,
                    'R' => Direction::East,
                    _ => panic!("Unknown character {c}"),
                }
            }).collect()
        }).collect()
    }
    
    fn part1(&self, input: &Self::I) -> Self::O {
        let mut acc = 0;
        let mut position = (1, 1);
        for moves in input {
            position = Self::move_keypad(position, moves);
            acc = acc * 10 + Self::to_digit(position);
        }
        acc.to_string()
    }
    
    fn part2(&self, input: &Self::I) -> Self::O {
        let mut acc = 0;
        let mut position = (1, 1);
        for moves in input {
            position = Self::move_keypad2(position, moves);
            acc = acc * 16 + Self::to_digit2(position);
        }
        String::from(format!("{acc:X}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day2::Day2};

    #[test]
    fn test_part1() {
        let test_input = r#"ULL
RRDDD
LURDL
UUUUD"#;
        let day2 = Day2;
        let parsed = day2.parse(test_input);
        assert_eq!("1985", day2.part1(&parsed));
    }

    #[test]
    fn test_part2() {
        let test_input = r#"ULL
RRDDD
LURDL
UUUUD"#;
        let day2 = Day2;
        let parsed = day2.parse(test_input);
        assert_eq!("5DB3", day2.part2(&parsed));
    }
}
