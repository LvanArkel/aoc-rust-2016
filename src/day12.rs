use std::collections::HashMap;

use crate::day::AocDay;

pub enum Value {
    Register(char),
    Constant(i32)
}

impl Value {
    fn parse(text: &str) -> Self {
        match text.parse() {
            Ok(val) => Self::Constant(val),
            Err(_) => Self::Register(text.chars().nth(0).unwrap())
        }
    }
}

pub enum Instruction {
    Cpy{ src: Value, dst: char },
    Inc(char),
    Dec(char),
    Jnz{ test: Value, offset: Value },
}

type State = HashMap<char, i32>;
fn initial_state() -> State {
    vec![
        ('a', 0),
        ('b', 0),
        ('c', 0),
        ('d', 0),
    ].into_iter().collect()
}

fn run_instructions(state: State, instructions: &Vec<Instruction>) -> State {
    let mut state = state;
    let mut program_counter = 0;

    let lookup = |value: &Value| {
        match value {
            Value::Register(r) => state[r],
            Value::Constant(v) => *v,
        }
    };

    while let Some(instruction) = instructions.get(program_counter) {
        match instruction {
            Instruction::Cpy { src, dst } => {
                let value = match src {
                    Value::Register(r) => state[r],
                    Value::Constant(v) => *v,
                };
                state.insert(*dst, value);
                program_counter += 1;
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
        }
    }
    state
}

pub struct Day12;
impl AocDay for Day12 {
    type I = Vec<Instruction>;

    type O = i32;

    fn filename() -> &'static str {
        "input/day12.txt"
    }

    fn parse(contents: &str) -> Self::I {
        contents.lines().map(|line| {
            let splitted: Vec<_> = line.split_ascii_whitespace().collect();
            match splitted[0] {
                "cpy" => Instruction::Cpy { 
                    src: Value::parse(splitted[1]),
                    dst: splitted[2].chars().nth(0).unwrap()
                },
                "jnz" => Instruction::Jnz { 
                    test: Value::parse(splitted[1]),
                    offset: Value::parse(splitted[2]),
                },
                "inc" => Instruction::Inc(splitted[1].chars().nth(0).unwrap()),
                "dec" => Instruction::Dec(splitted[1].chars().nth(0).unwrap()),
                _ => panic!("Unknown opcode {}", splitted[0])
            }
        }).collect()
    }

    fn part1(input: &Self::I) -> Self::O {
        let state = initial_state();
        let state = run_instructions(state, input);
        *state.get(&'a').unwrap()
    }

    fn part2(input: &Self::I) -> Self::O {
        let mut state = initial_state();
        state.insert('c', 1);
        let state = run_instructions(state, input);
        *state.get(&'a').unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::{day::AocDay, day12::Day12};

    #[test]
    fn test_part1() {
        let test_input = r#"cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"#;
        let parsed = Day12::parse(test_input);
        assert_eq!(42, Day12::part1(&parsed));
    }
}