pub mod day1;
pub mod day2;
mod utils;

pub trait AocDay {
    type I;
    type O;

    fn filename(&self) -> &'static str;

    fn parse(&self, contents: &str) -> Self::I;
    
    fn part1(&self, input: &Self::I) -> Self::O;
    fn part2(&self, input: &Self::I) -> Self::O;
}