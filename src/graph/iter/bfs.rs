use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::Graph;
use crate::graph::structure::node::Node;
use std::collections::{HashMap, VecDeque};

pub fn bfs<'a, G: Graph<'a>>(graph: &'a G, start: G::NId) -> Bfs<'a, G> {
    Bfs::new(graph, start)
}

pub struct Bfs<'a, G: Graph<'a>> {
    graph: &'a G,
    queue: VecDeque<G::NId>,
    parent: HashMap<G::NId, Option<G::EId>>,
}

impl<'a, G: Graph<'a>> Iterator for Bfs<'a, G> {
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

impl<'a, G: Graph<'a>> Bfs<'a, G> {
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
    use crate::graph::iter::bfs::bfs;
    use crate::graph::structure::mapgraph::MapGraph;
    use crate::graph::structure::vecgraph::StableVecGraph;
    use std::collections::HashMap;

    #[test]
    fn bfs_digraph() {
        let graph = StableVecGraph::from((
            vec![(); 7],
            vec![
                (0, 1, ()),
                (0, 2, ()),
                (1, 3, ()),
                (1, 4, ()),
                (2, 5, ()),
                (2, 6, ()),
                (6, 0, ()),
                (5, 2, ()),
                (5, 6, ()),
                (3, 2, ()),
                (5, 1, ()),
            ],
        ));

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
        let graph = MapGraph::from((
            vec![("a", ()), ("b", ()), ("c", ()), ("d", ()), ("e", ())],
            vec![("a", "b", ()), ("c", "a", ()), ("b", "d", ()), ("d", "e", ()), ("e", "c", ())],
        ));

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
