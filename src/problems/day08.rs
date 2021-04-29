use super::super::utils::read_strings_from_file;
use itertools::Itertools;
use std::str::FromStr;

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day08_1").expect("Failed to read inputs");
    println!("Problem 1: {:?}", problem_1(&strings));
    println!("Problem 2: {:?}", problem_2(&strings));
}

pub fn problem_1(strings: &Vec<String>) -> i32 {
    let instructions: Vec<Instruction> = strings
        .iter()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();
    println!("{:?}", instructions);
    let executor = CodeExecutor::new();
    let result = executor.execute(&instructions);
    println!("{:?}", result);
    match result {
        ExecutionResult::InfiniteLoop(state) => state.accumulator,
        ExecutionResult::Success(state) => state.accumulator,
    }
}

pub fn problem_2(strings: &Vec<String>) -> Option<i32> {
    let instructions: Vec<Instruction> = strings
        .iter()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();
    println!("{:?}", instructions);
    let code_generator = CodeCandidateGenerator{ instructions, index: 0 };
    let executor = CodeExecutor::new();
    for code_candidate in code_generator {
        println!("Candidate: {:?}", code_candidate);
        let result = executor.execute(&code_candidate);
        println!("{:?}", result);
        match result {
            ExecutionResult::Success(state) => { return Some(state.accumulator); },
            ExecutionResult::InfiniteLoop(state) => { continue; },
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq)]
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
enum ExecutionResult {
    Success(ExecutionState),
    InfiniteLoop(ExecutionState),
}

#[derive(Debug, PartialEq)]
struct CodeExecutor {}

impl CodeExecutor {
    fn new() -> CodeExecutor {
        Self {}
    }

    fn execute(&self, instructions: &Vec<Instruction>) -> ExecutionResult {
        let mut state = ExecutionState {
            accumulator: 0,
            address: 0,
        };
        let mut loop_tracker = vec![false; instructions.len()];
        loop {
            let new_state = instructions[state.address as usize].execute(&state);
            if new_state.address as usize >= instructions.len() {
                return ExecutionResult::Success(new_state);
            } else if loop_tracker[new_state.address as usize] {
                return ExecutionResult::InfiniteLoop(state);
            } else  {
                loop_tracker[new_state.address as usize] = true;
                state = new_state;
            }
        }
    }
}

struct CodeCandidateGenerator {
    instructions: Vec<Instruction>,
    index: usize,
}

impl Iterator for CodeCandidateGenerator {
    type Item = Vec<Instruction>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(instruction) = self.instructions.get(self.index) {
            match &instruction.operation[..] {
                "acc" => {
                    self.index += 1;
                },
                _ => {
                    // jmp or nop instruction
                    let mut mutated_code: Vec<Instruction> = self.instructions.iter().cloned().collect();
                    mutated_code[self.index] = if instruction.operation == "jmp" {
                        Instruction{operation: "nop".to_string(), argument: instruction.argument }
                    } else {
                        Instruction{operation: "jmp".to_string(), argument: instruction.argument }
                    };
                    self.index += 1;
                    return Some(mutated_code);
                }
            }
        }
        None
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

    #[test]
    fn test2() {
        let strings: Vec<String> = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(Some(8), problem_2(&strings));
    }
}
