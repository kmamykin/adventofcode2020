use super::super::utils::read_strings_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use std::num::ParseIntError;
use itertools::Itertools;
use std::path::StripPrefixError;

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day08_1").expect("Failed to read inputs");
    println!("Problem 1: {:?}", problem_1(&strings));
    // println!("Problem 2: {:?}", problem_2(&strings));
}

pub fn problem_1(strings: &Vec<String>) -> usize {
    let instructions: Vec<Instruction> = strings
        .iter()
        .map(|s| s.parse::<Instruction>().unwrap())
        .collect();
    println!("{:?}", instructions);
    5
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
        let parts: Option<(&str, &str)> = s
            .split(' ')
            .map(|s| s.trim())
            .collect_tuple();
        if let Some((a, b)) = parts {
            let operation = a.trim().to_string();
            let argument = b.parse::<i32>().unwrap();
            Ok(Instruction{ operation, argument })
        } else {
            Err("Can not parse string".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
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
        ].iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(5, problem_1(&strings));
    }
}
