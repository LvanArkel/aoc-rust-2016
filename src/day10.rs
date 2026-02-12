use core::panic;
use std::{collections::HashMap, sync::LazyLock};

use itertools::Itertools;
use regex::Regex;

use crate::{day::AocDay, utils::regex::capture_get_usize};

pub struct Day10;

static value_pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"value (\d+) goes to bot (\d+)"#
).unwrap());
static bot_pattern: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"bot (\d+) gives low to (bot \d+|output \d+) and high to (bot \d+|output \d+)"#
).unwrap());

#[derive(Clone, Copy)]
pub struct Bot {
    low: Destination,
    high: Destination,
}

#[derive(Clone, Copy)]
pub enum Destination {
    Bot(usize),
    Output(usize)
}

fn find_bot(
    initial_chips: &Vec<Vec<usize>>, 
    bots: &Vec<Bot>, 
    low_target: usize, 
    high_target: usize
) -> usize {
    let mut valid_bots: Vec<_> = initial_chips.iter()
        .enumerate()
        .filter(|(_, chips)| chips.len() == 2)
        .map(|(i, _)| i)
        .collect();
    let mut chips = initial_chips.to_owned();
    while let Some(head) = valid_bots.pop() {
        let bot = bots[head];
        let chip_box = &mut chips[head];
        let low = usize::min(chip_box[0], chip_box[1]);
        let high = usize::max(chip_box[0], chip_box[1]);

        if low == low_target && high == high_target {
            return head;
        }
        
        let low_dest = bot.low;
        match low_dest {
            Destination::Bot(bot_i) => {
                chips[bot_i].push(low);
                if chips[bot_i].len() == 2 {
                    valid_bots.push(bot_i);
                }
            },
            Destination::Output(_) => {},
        }

        let high_dest = bot.high;
        match high_dest {
            Destination::Bot(bot_i) => {
                chips[bot_i].push(high);
                if chips[bot_i].len() == 2 {
                    valid_bots.push(bot_i);
                }
            },
            Destination::Output(_) => {},
        }
    }
    panic!("Could not find solution")
}

impl AocDay for Day10 {
    type I = (Vec<Vec<usize>>, Vec<Bot>);

    type O = usize;

    fn filename(&self) -> &'static str {
        "input/day10.txt"
    }

    fn parse(&self, contents: &str) -> Self::I {
        let mut max_bots = 0;
        let mut value_lines: Vec<[usize; 2]> = Vec::new();
        let mut bot_lines: Vec<(usize, Bot)> = Vec::new();
        for line in contents.lines() {
            if let Some(capture) = value_pattern.captures(line) {
                let value = capture_get_usize(&capture, 1);
                let bot = capture_get_usize(&capture, 2);
                value_lines.push([value, bot]);
            } else if let Some(capture) = bot_pattern.captures(line) {
                let bot = capture_get_usize(&capture, 1);
                let low_str = capture.get(2).unwrap().as_str().split_once(" ").unwrap();
                let low = match low_str.0 {
                    "bot" => Destination::Bot(low_str.1.parse().unwrap()),
                    "output" => Destination::Output(low_str.1.parse().unwrap()),
                    _ => panic!("Invalid destination {}", low_str.0)
                };
                let high_str = capture.get(3).unwrap().as_str().split_once(" ").unwrap();
                let high = match high_str.0 {
                    "bot" => Destination::Bot(high_str.1.parse().unwrap()),
                    "output" => Destination::Output(high_str.1.parse().unwrap()),
                    _ => panic!("Invalid destination {}", high_str.0)
                };
                max_bots = max_bots.max(bot);
                bot_lines.push((bot, Bot { low, high }));
            } else {
                panic!("Could not parse line \"{line}\"");
            }
        }
        let bots: Vec<_> = bot_lines.into_iter()
            .sorted_by_key(|(i, _)| *i)
            .map(|(_, bot)| bot).collect();
        let mut chips = vec![Vec::new(); max_bots+1];
        for [value, bot] in value_lines {
            chips[bot].push(value);
        }
        (chips, bots)
    }

    fn part1(&self, input: &Self::I) -> Self::O {
        find_bot(&input.0, &input.1, 17, 61)
    }

    fn part2(&self, input: &Self::I) -> Self::O {
        let (initial_chips, bots) = input;
        let mut valid_bots: Vec<_> = initial_chips.iter()
            .enumerate()
            .filter(|(_, chips)| chips.len() == 2)
            .map(|(i, _)| i)
            .collect();
        let mut chips = initial_chips.to_owned();
        let mut outputs = [0; 3];
        while let Some(head) = valid_bots.pop() {
            let bot = bots[head];
            let chip_box = &mut chips[head];
            let low = usize::min(chip_box[0], chip_box[1]);
            let high = usize::max(chip_box[0], chip_box[1]);
            
            let low_dest = bot.low;
            match low_dest {
                Destination::Bot(bot_i) => {
                    chips[bot_i].push(low);
                    if chips[bot_i].len() == 2 {
                        valid_bots.push(bot_i);
                    }
                },
                Destination::Output(output_i) => {
                    if output_i <= 2 {
                        outputs[output_i] = low;
                    }
                },
            }

            let high_dest = bot.high;
            match high_dest {
                Destination::Bot(bot_i) => {
                    chips[bot_i].push(high);
                    if chips[bot_i].len() == 2 {
                        valid_bots.push(bot_i);
                    }
                },
                Destination::Output(output_i) => {
                    if output_i <= 2 {
                        outputs[output_i] = high;
                    }
                },
            }
        }
        return outputs[0] * outputs[1] * outputs[2]
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day10::{Day10, find_bot}};

    #[test]
    fn test_part1() {
        let input = r#"value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"#;

        let day = Day10;
        let (chips, bots) = day.parse(input);
        assert_eq!(2, find_bot(&chips, &bots, 2, 5));
    }
}