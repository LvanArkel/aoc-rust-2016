use std::collections::HashMap;

use itertools::Itertools;

use crate::day::AocDay;

pub struct Day22;

pub struct Node {
    size: u32,
    used: u32,
}

impl AocDay for Day22 {
    type I = HashMap<(u32, u32), Node>;

    type O = usize;

    fn filename() -> &'static str {
        "input/day22.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.lines().skip(2).map(|line| {
            let mut splitted = line.split_ascii_whitespace();
            let location_text = splitted.next().unwrap();
            let mut location_splitted = location_text.split("-").skip(1);
            let x = (&(location_splitted.next().unwrap())[1..]).parse().unwrap();
            let y = (&(location_splitted.next().unwrap())[1..]).parse().unwrap();
            let size_text = splitted.next().unwrap();
            let size = (&size_text[..size_text.len()-1]).parse().unwrap();
            let used_text = splitted.next().unwrap();
            let used = (&used_text[..used_text.len()-1]).parse().unwrap();
            let node = Node { size, used };
            ((x, y), node)
        }).collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        let keys = input.keys();
        keys.tuple_combinations().filter(|(a, b)| {
            let node_a = &input[a];
            let node_b = &input[b];
            
            (node_a.used > 0 && (node_b.size - node_b.used) >= node_a.used) ||
            (node_b.used > 0 && (node_a.size - node_a.used) >= node_b.used)
        }).count()
    }

    fn part2(input: &Self::I) -> Self::O {
        todo!()
    }
}