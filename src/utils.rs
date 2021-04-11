use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn read_strings_from_file(s: &str) -> Result<Vec<String>, Error> {
    let path = Path::new(s);
    let input = File::open(path)?;
    let list: Vec<String> = BufReader::new(input).lines().map(|l| l.unwrap()).collect();
    Ok(list)
}
