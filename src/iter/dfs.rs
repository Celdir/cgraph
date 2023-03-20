use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;
use crate::iter::traits::{Path, Traversal};
use std::collections::HashMap;

pub fn dfs<'a, G>(
    graph: &'a G,
    start: G::NId,
) -> Dfs<'a, G, impl Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool>
where
    G: Graph,
{
    Dfs::new(graph, start, |_, _| true)
}

pub fn dfs_where<'a, G, F>(graph: &'a G, start: G::NId, condition: F) -> Dfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    Dfs::new(graph, start, condition)
}

pub struct Dfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    graph: &'a G,
    stack: Vec<(G::NId, Option<G::EId>)>,
    parent: HashMap<G::NId, Option<G::EId>>,
    condition: F,
}

impl<'a, G, F> Iterator for Dfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    type Item = (
        Option<Edge<'a, G::NId, G::EId, G::E>>,
        Node<'a, G::NId, G::N>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let (mut node_id, mut edge_id_opt) = self.stack.pop()?;
        while self.parent.contains_key(&node_id) {
            (node_id, edge_id_opt) = self.stack.pop()?;
        }

        self.parent.insert(node_id, edge_id_opt);

        let adj: Vec<_> = self
            .graph
            .adj(node_id)?
            .filter(|(edge, node)| (self.condition)(&edge, &node))
            .collect();

        for (edge, node) in adj.iter().rev() {
            let next_id = node.id();
            if !self.parent.contains_key(&next_id) {
                self.stack.push((next_id, Some(edge.id())));
            }
        }

        let node = self.graph.node(node_id).unwrap();
        let parent_edge_opt = edge_id_opt.map(|edge_id| self.graph.edge(edge_id).unwrap());
        Some((parent_edge_opt, node))
    }
}

impl<'a, G, F> Traversal<'a, G> for Dfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    type StepItem = Self::Item;

    fn is_visited(&self, node_id: G::NId) -> bool {
        self.parent.contains_key(&node_id)
    }

    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        let &edge_id = self.parent.get(&id)?.as_ref()?;
        self.graph.edge(edge_id)
    }

    fn current_node(&self) -> Option<Node<'a, G::NId, G::N>> {
        self.graph.node(self.stack.last()?.0)
    }

    fn path_to(&mut self, target: G::NId) -> Option<Path<'a, G>> {
        while !self.parent.contains_key(&target) {
            self.next()?;
        }

        let mut path = Vec::new();
        let mut node_id_opt = Some(target);
        while node_id_opt.is_some() {
            let node_id = node_id_opt.unwrap();
            let node = self.graph.node(node_id).expect("node should exist");
            let edge = self.parent_edge(node_id);
            node_id_opt = edge.as_ref().map(|e| e.other(node_id));

            path.push((edge, node));
        }
        path.reverse();

        Some(Path::new(path))
    }
}

impl<'a, G, F> Dfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    fn new(graph: &'a G, start: G::NId, condition: F) -> Self {
        Dfs {
            graph,
            stack: vec![(start, None)],
            parent: HashMap::new(),
            condition: condition,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::traits::{Graph, OrdinalGraph, WithCapacity};
    use crate::graph::types::{DiListGraph, UnListGraph};
    use crate::iter::dfs::{dfs, dfs_where};
    use std::collections::HashMap;

    #[test]
    fn dfs_digraph() {
        let graph = DiListGraph::from_ordinal(
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
        );

        let expected_parents = HashMap::from([(1, 0), (2, 3), (3, 1), (4, 1), (5, 2), (6, 5)]);
        for (parent_edge, node) in dfs(&graph, 0) {
            match parent_edge {
                Some(edge) => {
                    let id = node.id();
                    let expected_parent_id = *expected_parents
                        .get(&id)
                        .expect("expected parents map should contain node id");
                    assert_eq!(
                        edge.other(id),
                        expected_parent_id,
                        "parent of {} should be {} but is {}",
                        id,
                        expected_parent_id,
                        edge.other(id)
                    )
                }
                _ => assert!(!expected_parents.contains_key(&node.id())),
            }
        }
    }

    #[test]
    fn dfs_ungraph() {
        // complete undirected graph with 5 nodes
        let mut graph = UnListGraph::with_capacity(5, 10);
        for _ in 0..5 {
            graph.insert_node(());
        }
        for i in 0..5 {
            for j in (i + 1)..5 {
                graph.insert_edge(i, j, ()).expect("nodes should exist");
            }
        }

        let expected_parents = HashMap::from([(0, 2), (1, 0), (3, 1), (4, 3)]);
        for (parent_edge, node) in dfs(&graph, 2) {
            match parent_edge {
                Some(edge) => {
                    let id = node.id();
                    let expected_parent_id = *expected_parents
                        .get(&id)
                        .expect("expected parents map should contain node id");
                    assert_eq!(
                        edge.other(id),
                        expected_parent_id,
                        "parent of {} should be {} but is {}",
                        id,
                        expected_parent_id,
                        edge.other(id)
                    )
                }
                _ => assert!(!expected_parents.contains_key(&node.id())),
            }
        }
    }

    #[test]
    fn dfs_even_edges() {
        let graph = DiListGraph::from_ordinal(
            vec![(); 7],
            vec![
                (0, 1, 2),
                (0, 2, 1),
                (1, 3, 2),
                (1, 4, 1),
                (2, 5, 2),
                (2, 6, 1),
                (6, 0, 1),
                (5, 2, 1),
                (5, 6, 2),
                (3, 2, 2),
                (5, 1, 1),
            ],
        );

        let expected_parents = HashMap::from([(1, 0), (2, 3), (3, 1), (5, 2), (6, 5)]);
        for (parent_edge, node) in dfs_where(&graph, 0, |&edge, _| *edge % 2 == 0) {
            match parent_edge {
                Some(edge) => {
                    let id = node.id();
                    let expected_parent_id = *expected_parents
                        .get(&id)
                        .expect("expected parents map should contain node id");
                    assert_eq!(
                        edge.other(id),
                        expected_parent_id,
                        "parent of {} should be {} but is {}",
                        id,
                        expected_parent_id,
                        edge.other(id)
                    )
                }
                _ => assert!(!expected_parents.contains_key(&node.id())),
            }
        }
    }
}
