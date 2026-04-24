use std::{collections::HashMap, sync::Arc};

use crate::{day::AocDay, day12::{Day12, Instruction, Value}};

type State = HashMap<char, i32>;
fn initial_state() -> State {
    vec![
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
    ].into_iter().collect()
}

fn run_instructions(
    initial_state: State, 
    instructions: &Vec<Instruction>,
    program_counter: usize,
    breakpoints: &Vec<usize>,
) -> State {
    let mut state = initial_state;
    let mut toggles: HashMap<usize, Instruction> = HashMap::new();
    let mut program_counter = program_counter;

    while let Some(instruction) = toggles.get(&program_counter).or(instructions.get(program_counter)) {
        if breakpoints.contains(&(program_counter+1)) {
            return state
        }
        match instruction {
            Instruction::Cpy { src, dst } => {
                if let Value::Register(dst) = dst{
                    let value = match src {
                        Value::Register(r) => state[r],
                        Value::Constant(v) => *v,
                    };
                    state.insert(*dst, value);
                    program_counter += 1;
                }
            },
            Instruction::Inc(reg) => {
                state.insert(*reg, state[reg] + 1);
                program_counter += 1;
            },
            Instruction::Dec(reg) => {
                state.insert(*reg, state[reg] - 1);
                program_counter += 1;
            },
            Instruction::Jnz { test, offset } => {
                let value = match test {
                    Value::Register(r) => state[r],
                    Value::Constant(v) => *v,
                };
                if value == 0 {
                    program_counter += 1;
                } else {
                    let off = match offset {
                        Value::Register(r) => state[r],
                        Value::Constant(v) => *v,
                    };
                    program_counter = program_counter.strict_add_signed(off as isize);
                }
            },
            Instruction::Tgl(offset) => {
                let value = program_counter as i32 + state[offset];
                if value >= 0 {
                    if let Some(instruction) = toggles.get(&(value as usize)).or(instructions.get(value as usize)) {
                        let new_instruction = match instruction {
                            Instruction::Inc(r) => Instruction::Dec(*r),
                            Instruction::Dec(r) => Instruction::Inc(*r),
                            Instruction::Tgl(r) => Instruction::Inc(*r),
                            Instruction::Cpy { src, dst } => Instruction::Jnz { test: *src, offset: dst.clone() },
                            Instruction::Jnz { test, offset } => Instruction::Cpy { src: test.clone(), dst: offset.clone()},
                        };
                        toggles.insert(value as usize, new_instruction);
                    }
                }
                program_counter += 1;
            },
        }
    }
    state
}

fn run_with_overrides(
    initial_state: State, 
    instructions: &Vec<Instruction>,
) -> State {
    let mut state = initial_state.clone();
    let mut toggles: HashMap<usize, Instruction> = HashMap::new();
    let mut program_counter = 0;
    let run_until = 10000;

    while let Some(instruction) = toggles.get(&program_counter).or(instructions.get(program_counter)) {
        if (program_counter+1) >= run_until {
            let expected_state = run_instructions(initial_state.clone(), instructions, 0, &vec![run_until]);
            println!("PC: {program_counter} state: {state:?}");
            println!("Expected: {expected_state:?}");
            println!("Is same {}", state == expected_state);
            panic!("Found run until");
        }

        match program_counter+1 {
            6 => {
                state.insert('a', state[&'a'] + state[&'c']);
                state.insert('c', 0);
                program_counter = 9 -1;
                continue;
            }
            5 => {
                state.insert('a', 
                    state[&'a'] + state[&'b'] * state[&'d']
                );
                state.insert('d', 0);
                state.insert('c', 0);
                program_counter = 10 -1;
                continue;
            }
            _ => {}
        }

        match instruction {
            Instruction::Cpy { src, dst } => {
                if let Value::Register(dst) = dst{
                    let value = match src {
                        Value::Register(r) => state[r],
                        Value::Constant(v) => *v,
                    };
                    state.insert(*dst, value);
                    program_counter += 1;
                }
            },
            Instruction::Inc(reg) => {
                state.insert(*reg, state[reg] + 1);
                program_counter += 1;
            },
            Instruction::Dec(reg) => {
                state.insert(*reg, state[reg] - 1);
                program_counter += 1;
            },
            Instruction::Jnz { test, offset } => {
                let value = match test {
                    Value::Register(r) => state[r],
                    Value::Constant(v) => *v,
                };
                if value == 0 {
                    program_counter += 1;
                } else {
                    let off = match offset {
                        Value::Register(r) => state[r],
                        Value::Constant(v) => *v,
                    };
                    program_counter = program_counter.strict_add_signed(off as isize);
                }
            },
            Instruction::Tgl(offset) => {
                let value = program_counter as i32 + state[offset];
                if value >= 0 {
                    if let Some(instruction) = toggles.get(&(value as usize)).or(instructions.get(value as usize)) {
                        let new_instruction = match instruction {
                            Instruction::Inc(r) => Instruction::Dec(*r),
                            Instruction::Dec(r) => Instruction::Inc(*r),
                            Instruction::Tgl(r) => Instruction::Inc(*r),
                            Instruction::Cpy { src, dst } => Instruction::Jnz { test: *src, offset: dst.clone() },
                            Instruction::Jnz { test, offset } => Instruction::Cpy { src: test.clone(), dst: offset.clone()},
                        };
                        toggles.insert(value as usize, new_instruction);
                    }
                }
                program_counter += 1;
            },
        }
    }
    state
}

pub struct Day23;
impl AocDay for Day23 {
    type I = Vec<Instruction>;

    type O = i32;

    fn filename() -> &'static str {
        "input/day23.txt"
    }

    fn parse(contents: &str) -> Self::I {
        Day12::parse(contents)
    }

    fn part1(input: &Self::I) -> Self::O {
        let mut state = initial_state();
        state.insert('a', 7);
        let end_state = run_instructions(state, input, 0, &vec![]);
        *end_state.get(&'a').unwrap()
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut state = initial_state();
        state.insert('a', 12);
        let end_state = run_with_overrides(state, input);
        *end_state.get(&'a').unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day23::{Day23, run_instructions}};

    #[test]
    fn test_day1() {
        let input = r#"cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a"#;

        let parsed = Day23::parse(input);
        assert_eq!(3, Day23::part1(&parsed));
    }
}