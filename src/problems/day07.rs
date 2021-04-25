use super::super::utils::read_strings_from_file;
use graphlib::{Graph, VertexId};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

type RulesGraph = Graph<String>;

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day07_1").expect("Failed to read inputs");
    println!("Problem 1: {:?}", problem_1(&strings));
    println!("Problem 2: {:?}", problem_2(&strings));
}

pub fn problem_1(strings: &Vec<String>) -> usize {
    let rules: Vec<(String, Vec<(usize, String)>)> = strings
        .iter()
        .map(|s| parse_rule(s).unwrap())
        .map(|(s, v)| (s.to_string(), parse_contained_bags(v)))
        .collect();
    println!("{:?}", rules);

    let graph = build_graph(&rules);
    let shiny_gold = find_node_in_graph(&graph, "shiny gold").unwrap();
    println!("{:?}", shiny_gold);
    let parents = find_all_parents_of_node(&graph, &shiny_gold);
    println!("{:?}: {:?}", parents.len(), parents);
    parents.len()
}

pub fn problem_2(strings: &Vec<String>) -> usize {
    let rules: Vec<(String, Vec<(usize, String)>)> = strings
        .iter()
        .map(|s| parse_rule(s).unwrap())
        .map(|(s, v)| (s.to_string(), parse_contained_bags(v)))
        .collect();

    let graph = build_graph(&rules);
    let shiny_gold = find_node_in_graph(&graph, "shiny gold").unwrap();
    let n_bags: usize = calculate_number_of_bags(&graph, &shiny_gold);
    n_bags
}

fn calculate_number_of_bags(graph: &RulesGraph, node: &VertexId) -> usize {
    // if no children return 1
    // if children, for each child return sum(weight * calculate_number_of_bags(child))
}

fn find_all_parents_of_node<'a>(graph: &'a RulesGraph, node: &'a VertexId) -> Vec<&'a VertexId> {
    let mut queue: Vec<&VertexId> = Vec::new();
    let mut parents: HashSet<&VertexId> = HashSet::new();
    queue.push(node);
    while let Some(v) = queue.pop() {
        if v != node {
            parents.insert(v);
        }
        graph.in_neighbors(v).for_each(|v| queue.push(v));
    }
    parents.into_iter().collect()
}

fn find_node_in_graph(graph: &RulesGraph, node_label: &str) -> Option<VertexId> {
    graph
        .vertices()
        .find(|&n| graph.fetch(n).unwrap() == node_label)
        .cloned()
}

fn upsert_node(graph: &mut RulesGraph, node_label: &str) -> VertexId {
    if let Some(vertex) = find_node_in_graph(graph, node_label) {
        vertex
    } else {
        graph.add_vertex(node_label.to_string())
    }
}

fn build_graph(rules: &Vec<(String, Vec<(usize, String)>)>) -> RulesGraph {
    let mut graph: Graph<String> = Graph::new();
    for (p, chs) in rules.iter() {
        let id1 = upsert_node(&mut graph, p);
        for (w, ch) in chs.iter() {
            let id2 = upsert_node(&mut graph, ch);
            graph.add_edge(&id1, &id2);
            // if p == "shiny gold" || ch == "shiny gold" {
            //     println!("Building {:?}: {:?}, {:?}", p, chs, graph.has_edge(&id1, &id2));
            // }
        }
    }
    graph
}

fn parse_rule(string: &str) -> Option<(&str, &str)> {
    string
        .split("bags contain")
        .map(|s| s.trim())
        .collect_tuple()
}

fn parse_contained_bags(string: &str) -> Vec<(usize, String)> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
    }
    REGEX
        .captures_iter(string)
        .map(|cap| {
            let num = cap[1].parse().unwrap();
            let name = cap[2].trim().to_string();
            (num, name)
        })
        .collect()
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test1() {
        let strings = vec![
            "muted lime bags contain 1 wavy lime bag, 1 vibrant green bag, 3 light yellow bags.",
            "light red bags contain 2 clear indigo bags, 3 light lime bags.",
            "wavy beige bags contain 4 faded chartreuse bags.",
        ];
        assert_eq!(
            Some((
                "muted lime",
                "1 wavy lime bag, 1 vibrant green bag, 3 light yellow bags."
            )),
            parse_rule(strings[0])
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            vec![
                (2usize, "clear indigo".to_string()),
                (3usize, "light lime".to_string())
            ],
            parse_contained_bags(" 2 clear indigo bags, 3 light lime bags.")
        )
    }

    #[test]
    fn test_problem_2() {
        let strings = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ];
        assert_eq!(126, problem_2(&strings));
    }
}
