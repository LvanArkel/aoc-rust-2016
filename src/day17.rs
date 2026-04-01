use std::collections::VecDeque;

use crate::{day::AocDay, utils::direction::Direction};

pub struct Day17;

impl AocDay for Day17 {
    type I = String;

    type O = String;

    fn filename() -> &'static str {
        "input/day17.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.to_string()
    }

    fn part1(input: &Self::I) -> Self::O {
        let mut open_set = VecDeque::new();
        open_set.push_back(("".to_string(), 0, 0));
        let destination = (3, 3);

        let directions = vec![
            (Direction::South, 'U'),
            (Direction::North, 'D'),
            (Direction::West, 'L'),
            (Direction::East, 'R'),
        ];

        while let Some((path, x, y)) = open_set.pop_front() {
            if (x, y) == destination {
                return path.to_string()
            }

            let key = format!("{input}{path}");
            let hash = md5::compute(&key);
            let hash_text = format!("{hash:x}");

            let new_states = hash_text
                .chars()
                .zip(directions.iter())
                .filter_map(|(hash_c, (direction, direction_letter))| {
                    let open = hash_c >= 'b';
                    let offset = direction.offset();
                    let new_position = (
                        x + offset.0,
                        y + offset.1
                    );
                    let in_bounds = new_position.0 >= 0 && new_position.0 < 4 && new_position.1 >= 0 && new_position.1 < 4;

                    if open && in_bounds {
                        Some((
                            format!("{path}{direction_letter}"),
                            new_position.0,
                            new_position.1,
                        ))
                    } else {
                        None
                    }
            });

            open_set.extend(new_states);
        }
        panic!("No solution found")
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut open_set = VecDeque::new();
        open_set.push_back(("".to_string(), 0, 0));
        let destination = (3, 3);

        let directions = vec![
            (Direction::South, 'U'),
            (Direction::North, 'D'),
            (Direction::West, 'L'),
            (Direction::East, 'R'),
        ];

        let mut longest_path = 0;

        while let Some((path, x, y)) = open_set.pop_front() {
            if (x, y) == destination {
                longest_path = longest_path.max(path.len());
                continue;
            }

            let key = format!("{input}{path}");
            let hash = md5::compute(&key);
            let hash_text = format!("{hash:x}");

            let new_states = hash_text
                .chars()
                .zip(directions.iter())
                .filter_map(|(hash_c, (direction, direction_letter))| {
                    let open = hash_c >= 'b';
                    let offset = direction.offset();
                    let new_position = (
                        x + offset.0,
                        y + offset.1
                    );
                    let in_bounds = new_position.0 >= 0 && new_position.0 < 4 && new_position.1 >= 0 && new_position.1 < 4;

                    if open && in_bounds {
                        Some((
                            format!("{path}{direction_letter}"),
                            new_position.0,
                            new_position.1,
                        ))
                    } else {
                        None
                    }
            });

            open_set.extend(new_states);
        }
        format!("{longest_path}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day17::Day17};

    #[test]
    fn test_part1() {
        assert_eq!("DDRRRD", Day17::part1(&"ihgpwlah".to_string()));
        assert_eq!("DDUDRLRRUDRD", Day17::part1(&"kglvqrro".to_string()));
        assert_eq!("DRURDRUDDLLDLUURRDULRLDUUDDDRR", Day17::part1(&"ulqzkmiv".to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!("370", Day17::part2(&"ihgpwlah".to_string()));
        assert_eq!("492", Day17::part2(&"kglvqrro".to_string()));
        assert_eq!("830", Day17::part2(&"ulqzkmiv".to_string()));
    }
}
