use super::super::utils::read_strings_from_file;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;

fn to_unique_chars(strs: &Vec<&str>) -> Vec<char> {
    strs.iter().flat_map(|&s| s.chars()).unique().collect()
}

fn to_intersection_of_chars(strs: &Vec<&str>) -> HashSet<char> {
    let hss: Vec<HashSet<char>> = strs
        .iter()
        .map(|&s| -> HashSet<char> { s.chars().collect() })
        .collect();
    //println!("Vec of HashSets: {:?}", hss);
    // reducing multiple HashSets to an intersection of all
    hss.iter().skip(1).fold(hss[0].clone(), |acc, h| {
        //println!("ACC: {:?}, el: {:?}", acc, h);
        acc.intersection(h).cloned().collect()
    })
}

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day06_1").expect("Failed to read inputs");
    let groups: Vec<Vec<&str>> = strings
        .iter()
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
        .collect();
    let anyone_answers: Vec<Vec<char>> =
        groups.iter().map(|group| to_unique_chars(group)).collect();
    println!("{:?}", strings);
    println!("{:?}", groups);
    println!("{:?}", anyone_answers);
    let n_anyone_answers_total: usize = anyone_answers.iter().map(|x| x.len()).sum();
    println!("Problem 1 solution: {:?}", n_anyone_answers_total);
    let everyone_answers: Vec<HashSet<char>> = groups
        .iter()
        .map(|group| to_intersection_of_chars(group))
        .collect();
    let n_everyone_answers_total: usize = everyone_answers.iter().map(|x| x.len()).sum();
    println!("{:?}", everyone_answers);
    println!("Problem 2 solution: {:?}", n_everyone_answers_total);
}
