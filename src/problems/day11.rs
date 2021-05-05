use super::super::utils::{parse_as, read_strings_from_file};

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day11_1").expect("Failed to read inputs");
    println!("Problem 1: {:?}", problem_1(&strings));
    println!("Problem 2: {:?}", problem_2(&strings));
}

pub fn problem_1(strings: &Vec<String>) -> u64 {
    println!("{:?}", strings);
    1
}

pub fn problem_2(strings: &Vec<String>) -> u64 {
    println!("{:?}", strings);
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_sum_1() {}

    #[test]
    fn test2() {
        let strings: Vec<String> = vec![
            "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
            "127", "219", "299", "277", "309", "576",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(62, problem_2(&strings));
    }
}
