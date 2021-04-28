use super::super::utils::read_strings_from_file;
use itertools::Itertools;
use std::str::FromStr;

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day08_1").expect("Failed to read inputs");
    println!("Problem 1: {:?}", problem_1(&strings));
    // println!("Problem 2: {:?}", problem_2(&strings));
}

pub fn problem_1(strings: &Vec<String>) -> i32 {
    let instructions: Vec<Instruction> = strings
        .iter()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();
    println!("{:?}", instructions);
    let mut executor = CodeExecutor::new(instructions);
    executor.execute();
    println!("{:?}", executor.state);
    executor.state.accumulator
}

pub fn problem_2(strings: &Vec<String>) -> usize {
    let rules: Vec<Instruction> = strings
        .iter()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();
    1
}

#[derive(Debug, PartialEq)]
struct Instruction {
    operation: String,
    argument: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Option<(&str, &str)> = s.split(' ').map(|s| s.trim()).collect_tuple();
        if let Some((a, b)) = parts {
            let operation = a.trim().to_string();
            let argument = b.parse::<i32>().unwrap();
            Ok(Instruction {
                operation,
                argument,
            })
        } else {
            Err("Can not parse string".to_string())
        }
    }
}

impl Instruction {
    fn execute(&self, state: &ExecutionState) -> ExecutionState {
        match &self.operation[..] {
            "acc" => ExecutionState {
                accumulator: state.accumulator + self.argument,
                address: state.address + 1,
            },
            "jmp" => ExecutionState {
                accumulator: state.accumulator,
                address: state.address + self.argument,
            },
            "nop" => ExecutionState {
                accumulator: state.accumulator,
                address: state.address + 1,
            },
            _ => ExecutionState {
                accumulator: state.accumulator,
                address: state.address,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ExecutionState {
    accumulator: i32,
    address: i32,
}
impl ExecutionState {}

#[derive(Debug, PartialEq)]
struct CodeExecutor {
    instructions: Vec<Instruction>,
    loop_tracker: Vec<bool>,
    state: ExecutionState,
}

impl CodeExecutor {
    fn new(instructions: Vec<Instruction>) -> CodeExecutor {
        let loop_tracker = vec![false; instructions.len()];
        let state = ExecutionState {
            accumulator: 0,
            address: 0,
        };
        Self {
            instructions,
            loop_tracker,
            state,
        }
    }

    fn execute(&mut self) -> () {
        loop {
            let new_state = self.instructions[self.state.address as usize].execute(&self.state);
            if self.loop_tracker[new_state.address as usize] {
                break;
            } else {
                self.loop_tracker[new_state.address as usize] = true;
                self.state = new_state;
            }
        }
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strings: Vec<String> = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(5, problem_1(&strings));
    }
}
