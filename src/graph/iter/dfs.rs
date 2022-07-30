use crate::graph::structure::graph::{Graph};
use crate::graph::structure::node::{Node};
use crate::graph::structure::edge::{Edge};
use std::collections::{HashMap, VecDeque};

pub fn dfs<'a, G: Graph<'a>>(graph: &'a G, start: <G as Graph<'a>>::NId) -> Dfs<'a, G> {
    Dfs::new(graph, start)
}

pub struct Dfs<'a, G: Graph<'a>> {
    graph: &'a G,
    stack: Vec<<G as Graph<'a>>::NId>,
    parent: HashMap<<G as Graph<'a>>::NId, Option<<G as Graph<'a>>::EId>>,
}

impl<'a, G: Graph<'a>> Iterator for Dfs<'a, G> {
    type Item = (Option<Edge<'a, <G as Graph<'a>>::NId, <G as Graph<'a>>::EId, <G as Graph<'a>>::E>>, Node<'a, <G as Graph<'a>>::NId, <G as Graph<'a>>::N>);

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
        let parent_edge_opt = self.parent.get(&node_id).unwrap().map(|edge_id| self.graph.edge(edge_id).unwrap());
        Some((parent_edge_opt, node))
    }
}

impl<'a, G: Graph<'a>> Dfs<'a, G> {
    fn new(graph: &'a G, start: <G as Graph<'a>>::NId) -> Dfs<'a, G> {
        Dfs{
            graph,
            stack: vec![start,],
            parent: HashMap::new(),
        }
    }
}
