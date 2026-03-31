use std::{fmt::Display, fs, time::Instant};

pub trait AocDay {
    type I;
    type O : Display;

    fn filename() -> &'static str;

    fn parse(contents: &str) -> Self::I;
    
    fn part1(input: &Self::I) -> Self::O;
    fn part2(input: &Self::I) -> Self::O;

    fn run() {
        let filename = Self::filename();
        let contents = fs::read_to_string(filename).unwrap();

        let parsed = Self::parse(&contents);
        
        let part1_start = Instant::now();
        let part1 = Self::part1(&parsed);
        let part1_end = Instant::now();
        let part1_duration = part1_end - part1_start;
        println!("Part 1: {part1}, in ({part1_duration:?})");

        let part2_start = Instant::now();
        let part2 = Self::part2(&parsed);
        let part2_end = Instant::now();
        let part2_duration = part2_end - part2_start;
        println!("Part 1: {part2}, in ({part2_duration:?})");
    }
}