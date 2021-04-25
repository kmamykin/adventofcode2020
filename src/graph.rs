use std::collections::HashMap;

pub trait GraphNode {
    fn key(&self) -> &str;
}

pub trait GraphEdge {}

pub struct Graph<N, E> {
    nodes: Vec<N>,
    edges: Vec<E>,
    nodes_index: HashMap<String, usize>,
}
impl<N: GraphNode, E: GraphEdge> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Self {
            nodes: Vec::<N>::new(),
            edges: Vec::<E>::new(),
            nodes_index: HashMap::<String, usize>::new(),
        }
    }

    pub fn add_node(&mut self, node: N) {
        self.nodes_index
            .insert(node.key().to_string(), self.nodes.len());
        self.nodes.push(node);
    }

    pub fn get_node_by_key(&self, key: &str) -> Option<&N> {
        self.nodes_index.get(key).and_then(|&i| self.nodes.get(i))
    }
}

mod example {
    use crate::graph::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ExampleNode {
        pub text: String,
    }
    impl GraphNode for ExampleNode {
        fn key(&self) -> &str {
            &self.text[..]
        }
    }

    pub struct ExampleEdge {
        weight: usize,
    }
    impl GraphEdge for ExampleEdge {}

    pub type ExampleGraph = Graph<ExampleNode, ExampleEdge>;
}

#[cfg(test)]
mod unit_tests {
    use super::example::*;
    // use super::*;

    #[test]
    fn built_a_simple_graph() {
        let mut graph = ExampleGraph::new();
        let node1 = ExampleNode {
            text: String::from("node1"),
        };
        let node2 = ExampleNode {
            text: String::from("node2"),
        };
        graph.add_node(node1);
        graph.add_node(node2);
        assert_eq!("node1", graph.get_node_by_key("node1").unwrap().text);
        assert_eq!("node2", graph.get_node_by_key("node2").unwrap().text);
    }
}
