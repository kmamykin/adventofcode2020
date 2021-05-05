use super::super::utils::{read_strings_from_file, parse_as};

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day09_1").expect("Failed to read inputs");
    println!("Problem 1: {:?}", problem_1(&strings, 25));
    println!("Problem 2: {:?}", problem_2(&strings));
}

pub fn problem_1(strings: &Vec<String>, preamble_len: usize) -> u64 {
    let numbers: Vec<u64> = parse_as::<u64>(strings);
    println!("{:?}", numbers);
    let n = first_number_that_is_not_sum(&numbers, preamble_len).unwrap();
    n
}

pub fn problem_2(strings: &Vec<String>) -> Option<i32> {
    let numbers: Vec<u64> = parse_as::<u64>(strings);
    println!("{:?}", numbers);
    None
}


#[derive(Debug)]
struct CrossSums {
    numbers: Vec<u64>,
}

impl CrossSums {
    fn new(preamble: &[u64]) -> Self {
        let numbers: Vec<u64> = preamble.iter().cloned().collect();
        Self {
            numbers
        }
    }

    fn each_sum(&self) -> impl Iterator<Item=u64> {
        let mut pairs: Vec<u64> = Vec::new();
        for i in 0..self.numbers.len() {
            for j in i+1..self.numbers.len() {
                pairs.push(self.numbers[i] + self.numbers[j]);
            }
        }
        pairs.into_iter()
    }

    fn contains(&self, n: u64) -> bool {
        self.each_sum()
            .any(|sum| sum == n)
    }

    fn push(&mut self, n: u64) -> &mut Self {
        self.numbers.remove(0);
        self.numbers.push(n);
        self
    }
}

fn first_number_that_is_not_sum(numbers: &Vec<u64>, preamble_len: usize) -> Option<u64> {
    let mut sums = CrossSums::new(&numbers[..preamble_len]);
    for i in preamble_len..numbers.len() {
        // println!("Testing numbers[{:?}] = {:?}", i, numbers[i]);
        if sums.contains(numbers[i]) {
            // println!("Sums {:?} contains {:?}", sums, numbers[i]);
            sums.push(numbers[i]);
        } else {
            // println!("Sums {:?} does not contain {:?}", sums, numbers[i]);
            return Some(numbers[i]);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_sum_1(){
        let arr: [u64; 5] = [1, 2, 3, 4, 5];
        let sums = CrossSums::new(&arr);
        assert!(sums.contains(6));
        assert!(!sums.contains(10));
    }
    #[test]
    fn test1() {
        let strings: Vec<String> = vec![
            "35",
            "20",
            "15",
            "25",
            "47",
            "40",
            "62",
            "55",
            "65",
            "95",
            "102",
            "117",
            "150",
            "182",
            "127",
            "219",
            "299",
            "277",
            "309",
            "576",
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(127, problem_1(&strings, 5));
    }

    #[test]
    fn test2() {
        assert_eq!(1, 1);
    }
}
