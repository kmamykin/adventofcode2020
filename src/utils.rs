use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn read_strings_from_file(s: &str) -> Result<Vec<String>, Error> {
    let path = Path::new(s);
    let input = File::open(path)?;
    let list: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    Ok(list)
}

pub fn parse_as<T>(strings: &Vec<String>) -> Vec<T>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let results: Vec<T> = strings
        .iter()
        .map(|s| T::from_str(s).unwrap())
        .collect();
    results
}
