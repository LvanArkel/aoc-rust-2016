use crate::day::AocDay;

pub struct Day5;

impl AocDay for Day5 {
    type I = String;
    type O = String;

    fn filename() -> &'static str {
        "input/day5.txt"
    }

    fn parse(contents: &str) -> Self::I {
        String::from(contents)
    }

    fn part1(input: &Self::I) -> Self::O {
        (0..u32::MAX)
            .map(|i| {format!("{input}{i}")})
            .filter_map(|text| { 
                let hash = md5::compute(text);
                let hash_text = format!("{hash:x}");
                if hash_text.starts_with("00000")  {
                    hash_text.chars().nth(5)
                } else {
                    None
                }
            })
            .take(8)
            .collect()
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut password: [Option<char>; 8] = [None; 8];
        for i in 0..u32::MAX {
            let text = format!("{input}{i}");
            let hash = md5::compute(text);
            let hash_text = format!("{hash:x}");
            if !hash_text.starts_with("00000") { continue; }
            let sixth = hash_text.chars().nth(5);
            let seventh = hash_text.chars().nth(6);
            match (sixth, seventh) {
                (Some(sixth), Some(seventh)) => {
                    match sixth.to_digit(10) {
                        Some(digit) if digit < 8 => {
                            if password[digit as usize].is_none() {
                                password[digit as usize] = Some(seventh);
                                if !password.contains(&None) {
                                    break;
                                }
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        password.into_iter().map(|x| x.unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day5::Day5};

    #[test]
    fn test_part1() {
        let input = "abc";
        assert_eq!("18f47a30", Day5::part1(&Day5::parse(input)))
    }
}