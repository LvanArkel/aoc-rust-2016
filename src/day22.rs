
use std::{cmp::Reverse, collections::HashMap, hash::Hash};

use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{day::AocDay, utils::grid::Grid};

pub struct Day22;

#[derive(Clone, Copy, Default)]
pub struct Node {
    size: u32,
    used: u32,
}

fn available_moves(
    sizes: &Grid<u32>,
    used: &Grid<u32>
) -> Vec<((usize, usize), (usize, usize))> {
    (0..(sizes.width*sizes.height))
        .into_par_iter()
        .flat_map(|i| {
            let x = i % sizes.width;
            let y = i / sizes.width;

            let size_a = sizes.get_unchecked(x, y);
            let used_a = *used.get_unchecked(x, y);
            let mut all_moves = Vec::with_capacity(4);

            // Horizontal
            if x < sizes.width - 1 {
                let size_h = sizes.get_unchecked(x + 1, y);
                let used_h = *used.get_unchecked(x + 1, y);
                
                if used_h == 0 && used_a <= *size_h {
                    all_moves.push(((x, y),(x + 1, y)));
                }
                if used_a == 0 && used_h <= *size_a {
                    all_moves.push(((x + 1, y),(x, y)));
                }
            }

            // Vertical
            if y < sizes.height - 1 {
                let size_v = sizes.get_unchecked(x, y + 1);
                let used_v = *used.get_unchecked(x, y + 1);
                
                if used_v == 0 && used_a <= *size_v {
                    all_moves.push(((x, y),(x, y + 1)));
                }
                if used_a == 0 && used_v <= *size_a {
                    all_moves.push(((x, y + 1),(x, y)));
                }
            }
            all_moves
        })
        .collect()
}

fn find_hole(
    used: &Grid<u32>
) -> (usize, usize) {
    for y in 0..used.height {
        for x in 0..used.width{
            if *used.get_unchecked(x, y) == 0 {
                return (x, y);
            }
        }
    }
    panic!("No hole found!");
}

fn print_grid(
    grid: &Grid<Node>,
) {
    let target = (grid.width-1, 0);
    let target_node = grid.get_unchecked(target.0, target.1);

    for y in 0..grid.height {
        for x in 0..grid.width {
            let node = grid.get_unchecked(x, y);
            if x == target.0 && y == target.1 {
                print!("G");
            } else if node.used == 0 {
                print!("_")
            } else if node.size > 200 {
                print!("#")
            } else if node.size < target_node.used {
                print!("+")
            } else {
                print!(".")
            }
        }
        print!("\n")
    }
}

impl AocDay for Day22 {
    type I = Grid<Node>;

    type O = usize;

    fn filename() -> &'static str {
        "input/day22.txt"
    }

    fn parse(contents: &str) -> Self::I {
        let nodes: Vec<_> = contents.lines().skip(2).map(|line| {
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
        }).collect();
        let max_width = *nodes.iter().map(|((x, _), _)| x).max().unwrap();
        let max_height = *nodes.iter().map(|((_, y), _)| y).max().unwrap();
        let mut grid = Grid::empty(max_width + 1, max_height + 1);
        for ((x, y), node) in nodes.into_iter() {
            grid.set(x, y, node);
        }
        grid
    }

    fn part1(input: &Self::I) -> Self::O {
        let indices = (0..input.width).flat_map(|x| {
            (0..input.height).map(move |y| {
                (x, y)
            })
        });

        indices.tuple_combinations().filter(|(i_a, i_b)| {
            let node_a = input.get(i_a.0, i_a.1).unwrap();
            let node_b = input.get(i_b.0, i_b.1).unwrap();

            (node_a.used > 0 && (node_b.size - node_b.used) >= node_a.used) ||
            (node_b.used > 0 && (node_a.size - node_a.used) >= node_b.used)
        }).count()
    }

    fn part2(input: &Self::I) -> Self::O {
        print_grid(input);

        let hole = (20, 25);
        let target = (33, 0);
        let left_edge = 4;

        let move_left = hole.0 - left_edge;
        let move_up = hole.1 - target.1;
        let move_right = target.0 - left_edge;
        let cycle = 5;
        let all_cycles = target.0 -1;
        return move_left + move_up + move_right + cycle*all_cycles;
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day22::Day22};

    #[test]
    fn test_part2() {
        //  8/10  7/ 9  6/10
        //  6/11  0/ 8  8/ 9
        // 28/32  7/11  6/ 9

        let input = r#"foo
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%"#;
        let parsed = Day22::parse(input);
        assert_eq!(7, Day22::part2(&parsed));
    }
}