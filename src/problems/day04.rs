use super::super::utils::read_strings_from_file;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct Passport {
    statements: Vec<(String, String)>,
}

fn is_year_in_range(from_year: i32, to_year: i32) -> impl Fn(Option<&str>) -> bool {
    let year_regex = Regex::new(r"^(\d{4})$").unwrap();
    move |maybe_value: Option<&str>| {
        maybe_value.map_or(false, |s| {
            year_regex.captures(s).map_or(false, |cap| {
                cap.get(1).map_or(false, |digits| {
                    let year = digits.as_str().parse::<i32>().unwrap();
                    if year >= from_year && year <= to_year {
                        true
                    } else {
                        false
                    }
                })
            })
        })
    }
}
fn is_height() -> impl Fn(Option<&str>) -> bool {
    let regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    move |maybe_value: Option<&str>| {
        maybe_value.map_or(false, |s| {
            regex.captures(s).map_or(false, |cap| {
                cap.get(1).map_or(false, |digits| {
                    cap.get(2).map_or(false, |unit| {
                        let val = digits.as_str().parse::<i32>().unwrap();
                        if unit.as_str() == "cm" {
                            if val >= 150 && val <= 193 {
                                true
                            } else {
                                false
                            }
                        } else if unit.as_str() == "in" {
                            if val >= 59 && val <= 76 {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                })
            })
        })
    }
}

fn is_matching_regex(re: &str) -> impl Fn(Option<&str>) -> bool {
    let regex = Regex::new(re).unwrap();
    move |maybe_value: Option<&str>| maybe_value.map_or(false, |s| regex.captures(s).is_some())
}

fn is_anything() -> impl Fn(Option<&str>) -> bool {
    move |maybe_value: Option<&str>| true
}

struct PassportValidator {}
impl PassportValidator {
    fn is_field_valid(&self, field: &str, value: Option<&str>) -> bool {
        match field {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            "byr" => is_year_in_range(1920, 2002)(value),
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            "iyr" => is_year_in_range(2010, 2020)(value),
            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030
            "eyr" => is_year_in_range(2010, 2030)(value),
            // hgt (Height) - a number followed by either cm or in:
            // If cm, the number must be at least 150 and at most 193.
            //     If in, the number must be at least 59 and at most 76.
            "hgt" => is_height()(value),
            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            "hcl" => is_matching_regex(r"^\#[0-9a-f]{6}$")(value),
            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            "ecl" => is_matching_regex(r"^amb|blu|brn|gry|grn|hzl|oth$")(value),
            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            "pid" => is_matching_regex(r"^\d{9}$")(value),
            "cid" => is_anything()(value),
            _ => true,
        }
    }

    fn is_valid(&self, passport: &Passport) -> bool {
        let passport_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
        let all_keys_valid = passport_keys
            .iter()
            .map(|&key| self.is_field_valid(key, passport.get(key)))
            .all(|v| v);
        all_keys_valid
    }
}

impl Passport {
    const RULES: i32 = 543;

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
    let validator = PassportValidator {};
    // byr
    assert!(!validator.is_field_valid("byr", None));
    assert!(!validator.is_field_valid("byr", Some("asdf")));
    assert!(!validator.is_field_valid("byr", Some("1919")));
    assert!(!validator.is_field_valid("byr", Some("2003")));
    assert!(validator.is_field_valid("byr", Some("1920")));
    assert!(validator.is_field_valid("byr", Some("2000")));
    assert!(validator.is_field_valid("byr", Some("2002")));

    // iyr
    assert!(!validator.is_field_valid("iyr", None));
    assert!(!validator.is_field_valid("iyr", Some("asdf")));
    assert!(!validator.is_field_valid("iyr", Some("2009")));
    assert!(!validator.is_field_valid("iyr", Some("2021")));
    assert!(validator.is_field_valid("iyr", Some("2010")));
    assert!(validator.is_field_valid("iyr", Some("2015")));
    assert!(validator.is_field_valid("iyr", Some("2020")));

    assert!(!validator.is_field_valid("hgt", Some("100in")));
    assert!(!validator.is_field_valid("hgt", Some("200cm")));
    assert!(validator.is_field_valid("hgt", Some("170cm")));

    assert!(validator.is_field_valid("hcl", Some("#1234ac")));
    assert!(!validator.is_field_valid("hcl", Some("#1234acf")));

    assert!(validator.is_field_valid("ecl", Some("oth")));
    assert!(!validator.is_field_valid("ecl", Some("123")));

    assert!(validator.is_field_valid("pid", Some("123456789")));
    assert!(!validator.is_field_valid("pid", Some("0234")));

    let p1 = Passport::from_statements(&vec![
        "byr:1921",
        "iyr:2020",
        "eyr:2020",
        "hcl:#1234ad",
        "ecl:amb",
        "cid:336",
        "hgt:182cm",
        "pid:533626984",
    ]);
    assert!(validator.is_valid(&p1));
    let strings = read_strings_from_file("./inputs/day04_1").expect("Failed to read inputs");
    let passports: Vec<Passport> = parse_strings_into_passports(&strings);
    let valid_passwords: Vec<&Passport> = passports
        .iter()
        .filter(|&p| validator.is_valid(p))
        .collect();

    println!("{:?}", passports);
    println!("{:?}", valid_passwords);
    println!("{:?}", valid_passwords.len());
}
