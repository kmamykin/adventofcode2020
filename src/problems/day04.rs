use super::super::utils::read_strings_from_file;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Passport {
    statements: Vec<(String, String)>,
}

impl Passport {
    fn from_statements(ss: &Vec<&str>) -> Passport {
        let statements = ss
            .iter()
            .map(|&s| {
                let pair: Vec<&str> = s.split(':').collect();
                (pair[0].to_string(), pair[1].to_string())
            })
            .collect();
        Self { statements }
    }

    fn get(&self, key: &str) -> Option<&str> {
        self.statements
            .iter()
            .find(|(k, v)| k == key)
            .map(|(_, v)| &v[..])
    }

    fn is_valid(&self) -> bool {
        let passport_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        let north_cole_credentials_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let has_all_passport_keys = passport_keys
            .iter()
            .map(|&k| self.get(k).is_some())
            .all(|v| v == true);
        let has_all_np_credentials_keys = north_cole_credentials_keys
            .iter()
            .map(|&k| self.get(k).is_some())
            .all(|v| v == true);
        has_all_np_credentials_keys || has_all_passport_keys
    }
}
fn parse_strings_into_passports(strings: &Vec<String>) -> Vec<Passport> {
    strings
        .iter()
        .flat_map(|s| s.split(' '))
        .batching(|it| {
            let mut batch: Vec<&str> = Vec::new();
            while let Some(el) = it.next() {
                if el.trim().len() == 0 {
                    return Some(batch);
                } else {
                    batch.push(el.trim());
                }
            }
            if batch.len() > 0 {
                Some(batch)
            } else {
                None
            }
        })
        .map(|vs| Passport::from_statements(&vs))
        .collect()
}
pub fn solve() {
    let strings = read_strings_from_file("./inputs/day04_1").expect("Failed to read inputs");
    let passports: Vec<Passport> = parse_strings_into_passports(&strings);
    let valid_passwords: Vec<&Passport> = passports.iter().filter(|p| p.is_valid()).collect();

    println!("{:?}", passports);
    println!("{:?}", valid_passwords);
    println!("{:?}", valid_passwords.len());
}
