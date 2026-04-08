use std::{collections::{HashMap, HashSet, VecDeque}, sync::LazyLock, usize};

use std::fmt::Debug;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use regex::Regex;

use crate::day::AocDay;

pub struct Day11;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum DeviceType {
    Chip, Generator
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Device {
    element: usize,
    kind: DeviceType,
}

impl Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.kind {
            DeviceType::Chip => "Ch",
            DeviceType::Generator => "Gn",
        };
        f.write_fmt(format_args!("{}{}", prefix, self.element))
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct State {
    elevator: usize,
    floors: [Vec<Device>; 4]
}

impl State {
    fn is_done(&self) -> bool {
        return self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }

    fn valid_state(&self) -> bool {
        self.floors.iter().all(|floor| {
            let (chips, generators): (Vec<_>, Vec<_>) = floor.iter().partition(|&device| device.kind == DeviceType::Chip);
            generators.is_empty() ||
            chips.iter().all(|chip: &Device| {
                generators.iter().any(|generator| chip.element == generator.element)
            })
        })
    }

    fn next_states(&self) -> Vec<State> {
        let current_floor = &self.floors[self.elevator];
        let single = current_floor.iter().map(|device| vec![device]);
        let doubles = current_floor.iter().combinations(2);
        let items: Vec<_> = single.chain(doubles).collect();

        let mut new_states = Vec::new();
        for devices in items {
            if self.elevator > 0 {
                // Elevator can go down
                let mut new_floors = self.floors.clone();
                new_floors[self.elevator].retain(|device| !&devices.contains(&device));
                for device in &devices {
                    new_floors[self.elevator - 1].push((*device).clone());
                }
                new_floors[self.elevator - 1].sort();
                new_states.push(State {
                    elevator: self.elevator - 1,
                    floors: new_floors,
                });
            }
            if self.elevator < 3 {
                // Elevator can go up
                let mut new_floors = self.floors.clone();
                new_floors[self.elevator].retain(|device| !&devices.contains(&device));
                for device in devices {
                    new_floors[self.elevator + 1].push(device.clone());
                }
                new_floors[self.elevator + 1].sort();
                new_states.push(State {
                    elevator: self.elevator + 1,
                    floors: new_floors,
                });
            }
        }
        new_states.into_iter().filter(|state| state.valid_state()).collect()
    }
}

static CHIP_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"(\w+)-compatible microchip"#
).unwrap());
static GENERATOR_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(
    r#"(\w+) generator"#
).unwrap());

impl AocDay for Day11 {
    type I = State;

    type O = usize;

    fn filename() -> &'static str {
        "input/day11.txt"
    }

    fn parse(contents: &str) -> Self::I {
        let mut element_cache: HashMap<&str, usize> = HashMap::new();
        let mut get_element_id = |name| {
            match element_cache.get(&name) {
                Some(id) => *id,
                None => {
                    let id = element_cache.len();
                    element_cache.insert(name, id);
                    id
                },
            }
        };

        let floors: [Vec<_>; 4] = contents.lines().map(|line| {
            let chips = CHIP_PATTERN.captures_iter(line)
                .map(|cap| {
                    let element = cap.get(1).unwrap().as_str();
                    (element, DeviceType::Chip)
                });
            let generators = GENERATOR_PATTERN.captures_iter(line)
                .map(|cap| {
                    let element = cap.get(1).unwrap().as_str();
                    (element, DeviceType::Generator)
                });
            chips.chain(generators).map(|(element, kind)| {
                let element_id = get_element_id(element);
                Device { element: element_id, kind }
            }).collect()
        }).collect_array().unwrap();
        State { elevator: 0, floors }
    }

    fn part1(input: &Self::I) -> Self::O {
        let initial_state = input.clone();
        let mut open_set = VecDeque::new();
        open_set.push_back(initial_state.clone());
        let mut visited: HashSet<State> = HashSet::new();

        let mut costs: HashMap<State, usize> = HashMap::new();
        costs.insert(initial_state, 0);

        let progress_bar = ProgressBar::no_length().with_style(
            ProgressStyle::with_template("[{elapsed_precise}] {pos:>7}").unwrap()
        );

        while let Some(state) = open_set.pop_front() {
            if visited.contains(&state) { continue; }

            
            progress_bar.inc(1);
            let cost = *costs.get(&state).unwrap();
            
            if state.is_done() { return cost }

            let next_states = state.next_states();
            for state in next_states {
                if let Some(existing_cost) = costs.get(&state) {
                    let new_cost = cost + 1;
                    if new_cost < *existing_cost {
                        costs.insert(state.clone(), cost + 1);
                    }
                } else {
                    costs.insert(state.clone(), cost + 1);
                }
                open_set.push_back(state);
            }


            visited.insert(state.clone());
                        
        }
        panic!("No solution found");
    }

    fn part2(input: &Self::I) -> Self::O {
        let max_element_id = input.floors.iter().flat_map(|floor| {
            floor.iter().map(|device| device.element)
        }).max().unwrap();
        let mut new_input = input.clone();
        new_input.floors[0].append(&mut vec![
            Device { element: max_element_id + 1, kind: DeviceType::Chip },
            Device { element: max_element_id + 2, kind: DeviceType::Chip },
            Device { element: max_element_id + 1, kind: DeviceType::Generator },
            Device { element: max_element_id + 2, kind: DeviceType::Generator },
        ]);
        Self::part1(&new_input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day11::{Day11, Device, State}};

    const TEST_INPUT: &str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn test_parsed() {
        let parsed = Day11::parse(TEST_INPUT);
        assert_eq!(2, parsed.floors[0].len());
        assert_eq!(1, parsed.floors[1].len());
        assert_eq!(1, parsed.floors[2].len());
        assert_eq!(0, parsed.floors[3].len());
    }

    #[test]
    fn test_valid_state() {
        let state = State {
            elevator: 2,
            floors: [
                vec![Device{ element: 0, kind: crate::day11::DeviceType::Chip}],
                vec![],
                vec![
                    Device{ element: 1, kind: crate::day11::DeviceType::Chip},
                    Device{ element: 0, kind: crate::day11::DeviceType::Generator},
                    Device{ element: 1, kind: crate::day11::DeviceType::Generator},
                ],
                vec![],
            ]
        };
        assert!(state.valid_state());
    }

    #[test]
    fn test_part1() {
        let parsed = Day11::parse(TEST_INPUT);
        assert_eq!(11, Day11::part1(&parsed));
    }
}
