use std::fs;
use std::time::Instant;

use aoc_rust_2016::day1::Day1;
use aoc_rust_2016::AocDay;

fn main() {
    let aoc_day = Day1;

    let filename = aoc_day.filename();
    let contents = fs::read_to_string(filename).unwrap();

    let parsed = aoc_day.parse(contents);
    
    let part1_start = Instant::now();
    let part1 = aoc_day.part1(&parsed);
    let part1_end = Instant::now();
    let part1_duration = part1_end - part1_start;
    println!("Part 1: {part1}, in ({part1_duration:?})");

    let part2_start = Instant::now();
    let part2 = aoc_day.part2(&parsed);
    let part2_end = Instant::now();
    let part2_duration = part2_end - part2_start;
    println!("Part 1: {part2}, in ({part2_duration:?})");
}
