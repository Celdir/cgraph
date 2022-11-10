use crate::graph::edge::Edge;
use crate::graph::graph::Graph;
use crate::graph::node::Node;
use std::collections::HashMap;

pub fn dfs<'a, G: Graph>(graph: &'a G, start: G::NId) -> Dfs<'a, G> {
    Dfs::new(graph, start)
}

pub struct Dfs<'a, G: Graph> {
    graph: &'a G,
    stack: Vec<G::NId>,
    parent: HashMap<G::NId, Option<G::EId>>,
}

impl<'a, G: Graph> Iterator for Dfs<'a, G> {
    type Item = (
        Option<Edge<'a, G::NId, G::EId, G::E>>,
        Node<'a, G::NId, G::N>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let node_id = self.stack.pop()?;
        if !self.parent.contains_key(&node_id) {
            self.parent.insert(node_id, None);
        }

        let adj: Vec<_> = self.graph.adj(node_id)?.collect();
        for (edge, node) in adj.iter().rev() {
            let next_id = node.id();
            if !self.parent.contains_key(&next_id) {
                self.parent.insert(next_id, Some(edge.id()));
                self.stack.push(next_id);
            }
        }

        let node = self.graph.node(node_id).unwrap();
        let parent_edge_opt = self
            .parent
            .get(&node_id)
            .unwrap()
            .map(|edge_id| self.graph.edge(edge_id).unwrap());
        Some((parent_edge_opt, node))
    }
}

impl<'a, G: Graph> Dfs<'a, G> {
    fn new(graph: &'a G, start: G::NId) -> Dfs<'a, G> {
        Dfs {
            graph,
            stack: vec![start],
            parent: HashMap::new(),
        }
    }
}
