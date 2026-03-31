use itertools::Itertools;

use crate::day::AocDay;

pub struct Day6;

impl AocDay for Day6 {
    type I = Vec<String>;

    type O = String;

    fn filename() -> &'static str {
        "input/day6.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.lines().map(|line| line.to_owned()).collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        let length = input[0].len();
        let mut iterators: Vec<_> = input.iter().map(|line| line.chars()).collect();
        (0..length).map(|_| {
            (0..input.len()).map(|i| {
                iterators[i].next().unwrap()
            }).counts()
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|x| x.0)
            .unwrap()
            .to_owned()
        }).collect()
    }

    fn part2(input: &Self::I) -> Self::O {
        let length = input[0].len();
        let mut iterators: Vec<_> = input.iter().map(|line| line.chars()).collect();
        (0..length).map(|_| {
            (0..input.len()).map(|i| {
                iterators[i].next().unwrap()
            }).counts()
            .iter()
            .max_by(|a, b| b.1.cmp(a.1))
            .map(|x| x.0)
            .unwrap()
            .to_owned()
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day6::Day6};

    #[test]
    fn test_part1() {
        let test_input = r#"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar"#;

        let parsed = Day6::parse(test_input);
        assert_eq!("easter", Day6::part1(&parsed));
    }
}