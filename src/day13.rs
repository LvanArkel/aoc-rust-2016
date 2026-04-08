use std::{cmp::Reverse, collections::{HashMap, HashSet}};

use priority_queue::PriorityQueue;

use crate::day::AocDay;

pub struct Day13;

fn valid_coordinate(n: usize, (x, y): (usize, usize)) -> bool {
    let sum = x*x + 3*x + 2*x*y + y + y*y + n;
    sum.count_ones() % 2 == 0
}

fn manhattan(start: (usize, usize), end: (usize, usize)) -> usize {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}

fn navigate(n: usize, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut open_set = PriorityQueue::new();
    open_set.push(start, Reverse(manhattan(start, end)));

    let mut visited = HashSet::new();

    let mut costs = HashMap::new();
    costs.insert(start, 0);

    while let Some((state, _)) = open_set.pop() {
        if visited.contains(&state) { continue; }
        visited.insert(state);

        let cost = *costs.get(&state).unwrap();
        if state == end {
            return cost;
        }

        // -X
        if state.0 > 0 {
            let new_state = (state.0 - 1, state.1);
            if valid_coordinate(n, new_state) {
                costs.insert(new_state, cost + 1);
                open_set.push(new_state, Reverse(cost + 1 + manhattan(new_state, end)));
            }
        }

        // +X
        let new_state = (state.0 + 1, state.1);
        if valid_coordinate(n, new_state) {
            costs.insert(new_state, cost + 1);
            open_set.push(new_state, Reverse(cost + 1 + manhattan(new_state, end)));
        }

        // -Y
        if state.1 > 0 {
            let new_state = (state.0, state.1 - 1);
            if valid_coordinate(n, new_state) {
                costs.insert(new_state, cost + 1);
                open_set.push(new_state, Reverse(cost + 1 + manhattan(new_state, end)));
            }
        }

        // +Y
        let new_state = (state.0, state.1 + 1);
        if valid_coordinate(n, new_state) {
            costs.insert(new_state, cost + 1);
            open_set.push(new_state, Reverse(cost + 1 + manhattan(new_state, end)));
        }
    }
    panic!("No solution found!")
}

impl AocDay for Day13 {
    type I = usize;

    type O = usize;

    fn filename() -> &'static str {
        "input/day13.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.parse().unwrap()
    }

    fn part1(input: &Self::I) -> Self::O {
        navigate(*input, (1, 1), (31, 39))
    }

    fn part2(input: &Self::I) -> Self::O {
        let start = (1, 1);
        let mut visited = HashSet::new();
        let mut open_set = vec![start];

        let mut costs = HashMap::new();
        costs.insert(start, 0_u32);

        while let Some(state) = open_set.pop() {
            if visited.contains(&state) { continue; }
            visited.insert(state);

            let cost = *costs.get(&state).unwrap();
            if cost == 50 { continue; }

            let mut new_states = vec![
                (state.0 + 1, state.1),
                (state.0, state.1 + 1)
            ];
            if state.0 > 0 {
                new_states.push((state.0  - 1, state.1));
            }
            if state.1 > 0 {
                new_states.push((state.0, state.1 - 1));
            }

            for new_state in new_states {
                if valid_coordinate(*input, new_state) {
                    match costs.get(&new_state) {
                        Some(old_cost) => {
                            if cost + 1 < *old_cost {
                                costs.insert(new_state, cost + 1);
                                visited.remove(&new_state);
                            }
                        },
                        None => {
                            costs.insert(new_state, cost + 1);
                        }
                    }
                    open_set.push(new_state);
                }
            }
        }
        visited.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::day13::navigate;

    #[test]
    fn test_part1() {
        assert_eq!(11, navigate(10, (1,1), (7, 4)))
    }
}