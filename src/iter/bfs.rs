use crate::graph::edge::Edge;
use crate::graph::traits::Graph;
use crate::graph::node::Node;
use std::collections::{HashMap, VecDeque};

pub fn bfs<'a, G: Graph>(graph: &'a G, start: G::NId) -> Bfs<'a, G> {
    Bfs::new(graph, start)
}

pub struct Bfs<'a, G: Graph> {
    graph: &'a G,
    queue: VecDeque<G::NId>,
    parent: HashMap<G::NId, Option<G::EId>>,
}

impl<'a, G: Graph> Iterator for Bfs<'a, G> {
    type Item = (
        Option<Edge<'a, G::NId, G::EId, G::E>>,
        Node<'a, G::NId, G::N>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let node_id = self.queue.pop_front()?;
        if !self.parent.contains_key(&node_id) {
            self.parent.insert(node_id, None);
        }

        for (edge, node) in self.graph.adj(node_id)? {
            let next_id = node.id();
            if !self.parent.contains_key(&next_id) {
                self.parent.insert(next_id, Some(edge.id()));
                self.queue.push_back(next_id);
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

impl<'a, G: Graph> Bfs<'a, G> {
    fn new(graph: &'a G, start: G::NId) -> Bfs<'a, G> {
        Bfs {
            graph,
            queue: VecDeque::from(vec![start]),
            parent: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::bfs::bfs;
    use crate::graph::traits::{Graph, KeyedGraph, OrdinalGraph, WithCapacity};
    use crate::graph::types::{DiListGraph, UnMapGraph};
    use std::collections::HashMap;

    #[test]
    fn bfs_digraph() {
        let mut graph = DiListGraph::with_capacity(7, 11);
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_edge(0, 1, ());
        graph.insert_edge(0, 2, ());
        graph.insert_edge(1, 3, ());
        graph.insert_edge(1, 4, ());
        graph.insert_edge(2, 5, ());
        graph.insert_edge(2, 6, ());
        graph.insert_edge(6, 0, ());
        graph.insert_edge(5, 2, ());
        graph.insert_edge(5, 6, ());
        graph.insert_edge(3, 2, ());
        graph.insert_edge(5, 1, ());

        let expected_parents = HashMap::from([(1, 0), (2, 0), (3, 1), (4, 1), (5, 2), (6, 2)]);
        for (parent_edge, node) in bfs(&graph, 0) {
            match parent_edge {
                Some(edge) => {
                    let expected_parent_id = *expected_parents
                        .get(&node.id())
                        .expect("expected parents map should contain node id");
                    assert_eq!(edge.origin(), expected_parent_id)
                }
                _ => assert!(!expected_parents.contains_key(&node.id())),
            }
        }
    }

    #[test]
    fn bfs_ungraph() {
        let mut graph = UnMapGraph::with_capacity(5, 5);
        graph.put_node("a", ());
        graph.put_node("b", ());
        graph.put_node("c", ());
        graph.put_node("d", ());
        graph.put_node("e", ());
        graph.insert_edge("a", "b", ());
        graph.insert_edge("c", "a", ());
        graph.insert_edge("b", "d", ());
        graph.insert_edge("d", "e", ());
        graph.insert_edge("e", "c", ());
 
        let expected_parents = HashMap::from([("b", "a"), ("c", "a"), ("d", "b"), ("e", "c")]);
        for (parent_edge, node) in bfs(&graph, "a") {
            match parent_edge {
                Some(edge) => {
                    let expected_parent_id = *expected_parents
                        .get(&node.id())
                        .expect("expected parents map should contain node id");
                    assert_eq!(edge.other(node.id()), expected_parent_id)
                }
                _ => assert!(!expected_parents.contains_key(&node.id())),
            }
        }
    }
}
