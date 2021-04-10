use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

fn read_numbers_from_file(s: &str) -> Result<Vec<i32>, Error> {
    let path = Path::new(s);
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let numbers: Vec<i32> = buffered
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.parse::<i32>().unwrap_or(0))
        .collect();
    Ok(numbers)
}

fn find_two_numbers_adding_to(numbers: &Vec<i32>, sum: i32) -> Option<(i32, i32)> {
    for n1 in numbers.iter() {
        for n2 in numbers.iter() {
            if n1 + n2 == sum {
                return Some((*n1, *n2));
            }
        }
    }
    None
}

fn find_three_numbers_adding_to(numbers: &Vec<i32>, sum: i32) -> Option<(i32, i32, i32)> {
    for n1 in numbers.iter() {
        for n2 in numbers.iter() {
            for n3 in numbers.iter() {
                if n1 + n2 + n3 == sum {
                    return Some((*n1, *n2, *n3));
                }
            }
        }
    }
    None
}

fn day01_problem01() -> Result<(), Error> {
    let numbers =
        read_numbers_from_file("./inputs/day01_1").expect("Could not find or parse the input file");
    let result01 = find_two_numbers_adding_to(&numbers, 2020).unwrap();
    println!("Problem1: {:?}", result01.0 * result01.1);
    let result02 = find_three_numbers_adding_to(&numbers, 2020).unwrap();
    println!("Problem2: {:?}", result02.0 * result02.1 * result02.2);
    Ok(())
}

fn main() -> Result<(), Error> {
    day01_problem01()
}
