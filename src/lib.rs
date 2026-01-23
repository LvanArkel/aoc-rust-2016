pub mod day1;

pub trait AocDay<I, O> {
    fn filename(&self) -> &'static str;

    fn parse(&self, contents: String) -> I;
    
    fn part1(&self, input: &I) -> O;
    fn part2(&self, input: &I) -> O;
}