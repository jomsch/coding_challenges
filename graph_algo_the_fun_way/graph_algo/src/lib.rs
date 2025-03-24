use std::collections::HashMap;

pub type Index = usize;
pub type Weight = f32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    source: Index,
    destination: Index,
    weight: Weight,
}
impl Edge {
    pub fn new(source: Index, destination: Index, weight: Weight) -> Self {
        Self {
            source,
            destination,
            weight,
        }
    }
}

pub struct Node {
    index: Index,
    edges: HashMap<Index, Edge>,
    label: Option<String>,
}

impl Node {
    pub fn new(index: Index, label: Option<String>) -> Self {
        Self {
            index,
            edges: HashMap::new(),
            label,
        }
    }

    pub fn num_edges(&self) -> usize {
        self.edges.len()
    }

    pub fn get_edge(&self, neighbor: Index) -> Option<&Edge> {
        self.edges.get(&neighbor)
    }

    pub fn add_edge(&mut self, neighbor: Index, weight: Weight) {
        self.edges
            .insert(neighbor, Edge::new(self.index, neighbor, weight));
    }

    pub fn remove_edge(&mut self, neighbor: Index) {
        self.edges.remove(&neighbor);
    }

    pub fn edges_iter(&self) -> impl Iterator<Item = Edge> {
        self.edges.iter().map(|(_, edge)| *edge)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GraphKind {
    Directed,
    Undirected,
}

impl GraphKind {
    pub fn is_unidrected(&self) -> bool {
        *self == GraphKind::Undirected
    }
}

pub struct Graph {
    nodes: Vec<Node>,
    kind: GraphKind,
}

impl Graph {
    pub fn new(num_nodes: usize, kind: GraphKind) -> Self {
        Self {
            nodes: (0..num_nodes)
                .into_iter()
                .map(|i| Node::new(i, None))
                .collect(),
            kind,
        }
    }

    pub fn get_edge(&self, source: Index, destination: Index) -> Option<&Edge> {
        self.nodes[source].get_edge(destination)
    }

    pub fn is_edge(&self, source: Index, destination: Index) -> bool {
        self.get_edge(source, destination).is_some()
    }

    pub fn edges_iter(&self) -> impl Iterator<Item = Edge> {
        self.nodes.iter().map(|node| node.edges_iter()).flatten()
    }

    pub fn insert_edge(&mut self, source: Index, destination: Index, weight: Weight) {
        self.nodes[source].add_edge(destination, weight);
        if self.kind.is_unidrected() {
            self.nodes[destination].add_edge(source, weight);
        }
    }

    pub fn remove_edge(&mut self, source: Index, destination: Index) {
        self.nodes[source].remove_edge(destination);
        if self.kind.is_unidrected() {
            self.nodes[destination].remove_edge(source);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut graph = Graph::new(5, GraphKind::Directed);
        graph.insert_edge(0, 1, 1.0);
        graph.insert_edge(0, 3, 1.0);
        graph.insert_edge(0, 4, 3.0);
        graph.insert_edge(1, 2, 2.0);
        graph.insert_edge(1, 4, 2.0);
        graph.insert_edge(3, 4, 3.0);
        graph.insert_edge(4, 2, 3.0);
        graph.insert_edge(4, 3, 3.0);

        assert_eq!(graph.get_edge(2, 1), None);
        assert_eq!(graph.get_edge(0, 1), Some(&Edge::new(0, 1, 1.0)));
        assert_eq!(graph.get_edge(1, 0), None);
    }
}
