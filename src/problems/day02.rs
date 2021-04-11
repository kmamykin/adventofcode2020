// use regex::Regex;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Password {
    password: String,
}

impl Password {
    fn count_occurrences(&self, ch: char) -> i32 {
        self.password
            .chars()
            .filter(|&c| c == ch)
            .count()
            .try_into()
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    min_occurrences: i32,
    max_occurrences: i32,
    character: char,
}

impl PasswordPolicy {
    fn satisfies(&self, password: &Password) -> bool {
        let actual_char_count = password.count_occurrences(self.character);
        actual_char_count >= self.min_occurrences && actual_char_count <= self.max_occurrences
    }
}

impl FromStr for PasswordPolicy {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let minmax: Vec<&str> = parts[0].split('-').collect();

        let min = minmax[0].parse::<i32>()?;
        let max = minmax[1].parse::<i32>()?;
        let char = parts[1].chars().nth(0).unwrap_or(' ');
        Ok(PasswordPolicy {
            min_occurrences: min,
            max_occurrences: max,
            character: char,
        })
    }
}

fn parse_password_policy_pair(s: &str) -> Option<(PasswordPolicy, Password)> {
    let parts: Vec<&str> = s.split(':').collect();
    let policy = parts[0].parse::<PasswordPolicy>().unwrap();
    let password = Password {
        password: parts[1].to_string(),
    };
    return Some((policy, password));
}

fn read_password_policy_pairs_from_file(s: &str) -> Result<Vec<(PasswordPolicy, Password)>, Error> {
    let path = Path::new(s);
    let input = File::open(path)?;
    let list: Vec<(PasswordPolicy, Password)> = BufReader::new(input)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| parse_password_policy_pair(&l).unwrap())
        .collect();
    Ok(list)
}
pub fn day02() {
    let p = "10-19 v".parse::<PasswordPolicy>().unwrap();
    assert_eq!('v', p.character);
    assert_eq!(10, p.min_occurrences);
    assert_eq!(19, p.max_occurrences);
    println!("{:?}", p);
    let list =
        read_password_policy_pairs_from_file("./inputs/day02_1").expect("Could not read file");
    let n_valid_passwords = list.iter().filter(|(pp, p)| pp.satisfies(&p)).count();
    println!("{:?}", n_valid_passwords);
    // list.iter()
    //     .for_each(|(pp, p)| println!("{:?} : {:?}", pp, p))
}
