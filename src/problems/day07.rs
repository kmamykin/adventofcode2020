use super::super::utils::read_strings_from_file;
use graphlib::{Graph, VertexId};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

type RulesGraph = Graph<String>;

pub fn solve() {
    let strings = read_strings_from_file("./inputs/day07_1").expect("Failed to read inputs");
    let rules: Vec<(String, Vec<(usize, String)>)> = strings
        .iter()
        .map(|s| parse_rule(s).unwrap())
        .map(|(s, v)| (s.to_string(), parse_contained_bags(v)))
        .collect();
    println!("{:?}", rules);

    let graph = build_graph(&rules);
    let shiny_gold = find_node_in_graph(&graph, "shiny gold");
    println!("{:?}", shiny_gold);
    let parents = find_all_parents_of_node(&graph, &shiny_gold.unwrap());
    println!("{:?}: {:?}", parents.len(), parents);
}

fn find_all_parents_of_node(graph: &RulesGraph, node: &VertexId) -> Vec<VertexId> {
    let mut queue: Vec<VertexId> = Vec::new();
    let mut parents: HashSet<VertexId> = HashSet::new();
    queue.push(node.to_owned());
    while let Some(v) = queue.pop() {
        if v != *node {
            parents.insert(v);
        }
        graph
            .in_neighbors(&v)
            .for_each(|&v| queue.push(v.to_owned()));
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
}
