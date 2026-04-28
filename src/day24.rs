use std::{collections::{HashMap, HashSet}, iter::once};

use itertools::Itertools;

use crate::day::AocDay;

pub struct Day24;

pub struct Node {
    value: Option<u32>,
    neighbours: Vec<(usize, usize, u32)>,
}

fn shortest_path(grid: &HashMap<(usize, usize), Node>, start: &(usize, usize), end: &(usize, usize)) -> u32 {
    let mut unvisited: Vec<_> = grid.keys().copied().collect();
    let mut distances: HashMap<_, _> = HashMap::new();
    for node in &unvisited {
        distances.insert(node.clone(), u32::MAX);
    }
    distances.insert(start.to_owned(), 0);

    while unvisited.len() > 0 {
        let min_pos = unvisited.iter().position_min_by_key(|node| distances[node]).unwrap();
        let node_coord = unvisited.remove(min_pos);
        let node = &grid[&node_coord];
        let node_distance = distances[&node_coord];
        if node_distance == u32::MAX {
            break;
        }
        // TODO: Terminate on infinite distance
        for (x, y, distance) in &node.neighbours {
            let neighbour_coord = (*x, *y);
            if unvisited.contains(&neighbour_coord)  {
                let new_distance = node_distance + distance;
                if new_distance < distances[&neighbour_coord] {
                    distances.insert(neighbour_coord, new_distance);
                }
            }
        }
    }
    distances[end]
}

impl AocDay for Day24 {
    type I = HashMap<(usize, usize), Node>;

    type O = u32;

    fn filename() -> &'static str {
        "input/day24.txt"
    }

    fn parse(contents: &str) -> Self::I {
        let lines: Vec<Vec<_>> = contents.lines().map(|line| line.chars().collect()).collect();
        let mut nodes = HashMap::new();
        for y in 1..lines.len() - 1 {
            let line = &lines[y];
            for x in 1..line.len()-1 {
                let c = line[x];
                if c == '#' { continue; }
                let mut neighbours = Vec::new();
                if lines[y][x-1] != '#' {
                    neighbours.push((x-1, y, 1));
                }
                if lines[y][x+1] != '#' {
                    neighbours.push((x+1, y, 1));
                }
                if lines[y-1][x] != '#' {
                    neighbours.push((x, y-1, 1));
                }
                if lines[y+1][x] != '#' {
                    neighbours.push((x, y+1, 1));
                }
                let value = c.to_digit(10);
                let node = Node {
                    value,
                    neighbours
                };
                nodes.insert((x, y), node);
            }
        }

        let all_nodes: Vec<_> = nodes.keys().map(|pos| pos.to_owned()).collect();
        for node_pos in all_nodes {
            let node = &nodes[&node_pos];
            if node.neighbours.len() == 2 && node.value.is_none() {
                let first_neighbour = node.neighbours[0];
                let second_neighbour = node.neighbours[1];
                let distance = first_neighbour.2 + second_neighbour.2;
                
                let first_node = nodes.get_mut(&(first_neighbour.0, first_neighbour.1)).unwrap();
                let first_node_i = first_node.neighbours.iter().position(|(x, y, _)| {
                    *x == node_pos.0 && *y == node_pos.1
                }).unwrap();
                first_node.neighbours.remove(first_node_i);
                first_node.neighbours.push((second_neighbour.0, second_neighbour.1, distance));
                
                let second_node = nodes.get_mut(&(second_neighbour.0, second_neighbour.1)).unwrap();
                let second_node_i = second_node.neighbours.iter().position(|(x, y, _)| {
                    *x == node_pos.0 && *y == node_pos.1
                }).unwrap();
                second_node.neighbours.remove(second_node_i);
                second_node.neighbours.push((first_neighbour.0, first_neighbour.1, distance));

                nodes.remove(&node_pos);
            }
        }
        nodes
    }

    fn part1(input: &Self::I) -> Self::O {
        let mut start = (0, 0);
        let mut destinations: Vec<_> = Vec::new();

        for (coord, node) in input {
            if let Some(value) = node.value {
                if value == 0 {
                    start = *coord;
                } else {
                    destinations.push(coord);
                }
            }
        }

        let all_distances: HashMap<_, _> = destinations.iter()
            .chain(once(&&start))
            .tuple_combinations()
            .flat_map(|(&a, &b)| {
            let distance = shortest_path(input, a, b);
            vec![((*a, *b), distance), ((*b, *a), distance)]
        }).collect();

        let destination_count = destinations.len();
        destinations.into_iter().permutations(destination_count).map(|sequence| {
            all_distances[&(start, *sequence[0])] +
            sequence.windows(2).map(|dests| {
                all_distances[&(*dests[0], *dests[1])]
            }).sum::<u32>()
        }).min().unwrap()
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut start = (0, 0);
        let mut destinations: Vec<_> = Vec::new();

        for (coord, node) in input {
            if let Some(value) = node.value {
                if value == 0 {
                    start = *coord;
                } else {
                    destinations.push(coord);
                }
            }
        }

        let all_distances: HashMap<_, _> = destinations.iter()
            .chain(once(&&start))
            .tuple_combinations()
            .flat_map(|(&a, &b)| {
            let distance = shortest_path(input, a, b);
            vec![((*a, *b), distance), ((*b, *a), distance)]
        }).collect();

        let destination_count = destinations.len();
        destinations.into_iter().permutations(destination_count).map(|sequence| {
            all_distances[&(start, *sequence[0])] +
            sequence.windows(2).map(|dests| {
                all_distances[&(*dests[0], *dests[1])]
            }).sum::<u32>() +
            all_distances[&(*sequence[sequence.len()-1], start)]
        }).min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day24::Day24};

    #[test]
    fn test_part1() {
        let input = r#"###########
#0.1.....2#
#.#######.#
#4.......3#
###########"#;
        let parsed = Day24::parse(input);
        
        // for (y, line) in input.lines().enumerate() {
        //     let line_str: String = line.char_indices().map(|(x, c)| {
        //         if parsed.contains_key(&(x, y)) {
        //             char::from_digit(parsed[&(x, y)].neighbours.len() as u32, 10).unwrap()
        //         } else {
        //             c
        //         }
        //     }).collect();
        //     println!("{line_str}");
        // }

        assert_eq!(5, parsed.len());
        assert_eq!(14, Day24::part1(&parsed));
    }
}