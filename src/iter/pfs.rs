use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;
use crate::iter::traits::{Path, Traversal};
use priority_queue::PriorityQueue;
use std::collections::{HashMap, VecDeque};

pub fn pfs<'a, G>(
    graph: &'a G,
    start: G::NId,
) -> Pfs<'a, G, impl Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool>
where
    G: Graph,
{
    Pfs::new(graph, start, |_, _| true)
}

pub fn pfs_where<'a, G, F>(graph: &'a G, start: G::NId, condition: F) -> Pfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    Pfs::new(graph, start, condition)
}

pub struct Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord,
    A: Fn(Option<P>, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    graph: &'a G,
    pq: PriorityQueue<G::NId, P>,
    parent: HashMap<G::NId, Option<G::EId>>,
    priority: HashMap<G::NId, Option<P>>,
    accumulator: A,
    condition: F,
}

impl<'a, G, P, A, F> Iterator for Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord,
    A: Fn(Option<P>, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    type Item = (
        Option<Edge<'a, G::NId, G::EId, G::E>>,
        Node<'a, G::NId, G::N>,
        Option<P>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let (node_id, priority) = self.pq.pop()?;
        let mut priority_opt = Some(priority);
        if !self.parent.contains_key(&node_id) {
            // node is start
            self.parent.insert(node_id, None);
            priority_opt = None;
        }
        self.priority.insert(node_id, priority);

        for (edge, node) in self.graph.adj(node_id)? {
            if (self.condition)(&edge, &node) {
                let next_id = node.id();

                if !self.priority.contains_key(&next_id) {
                    let next_priority = (self.accumulator)(priority_opt, &edge, &node);
                    let old_priority = pq.push_increase(next_id, next_priority);

                    match old_priority {
                        // update parent if priority is increased
                        Some(old_cost) if old_cost == next_cost => {}
                        _ => {
                            parent.insert(next_id, edge);
                        }
                    }
                }
            }
        }

        let node = self.graph.node(node_id).unwrap();
        let parent_edge_opt = self
            .parent
            .get(&node_id)
            .unwrap()
            .map(|edge_id| self.graph.edge(edge_id).unwrap());
        Some((parent_edge_opt, node, priority_opt))
    }
}

impl<'a, G, F> Traversal<'a, G> for Pfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    fn is_visited(&self, node_id: G::NId) -> bool {
        self.parent.contains_key(&node_id)
    }

    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        let &edge_id = self.parent.get(&id)?.as_ref()?;
        self.graph.edge(edge_id)
    }

    fn current_node(&self) -> Option<Node<'a, G::NId, G::N>> {
        self.graph.node(*self.queue.front()?)
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

impl<'a, G, F> Pfs<'a, G, F>
where
    G: Graph,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    fn new(graph: &'a G, start: G::NId, condition: F) -> Self {
        Pfs {
            graph,
            queue: VecDeque::from(vec![start]),
            parent: HashMap::new(),
            condition: condition,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::traits::{Graph, KeyedGraph, OrdinalGraph, WithCapacity};
    use crate::graph::types::{DiListGraph, UnMapGraph};
    use crate::iter::pfs::{pfs, pfs_where};
    use std::collections::HashMap;

    #[test]
    fn pfs_digraph() {
        let mut graph = DiListGraph::with_capacity(7, 11);
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_edge(0, 1, ()).expect("nodes should exist");
        graph.insert_edge(0, 2, ()).expect("nodes should exist");
        graph.insert_edge(1, 3, ()).expect("nodes should exist");
        graph.insert_edge(1, 4, ()).expect("nodes should exist");
        graph.insert_edge(2, 5, ()).expect("nodes should exist");
        graph.insert_edge(2, 6, ()).expect("nodes should exist");
        graph.insert_edge(6, 0, ()).expect("nodes should exist");
        graph.insert_edge(5, 2, ()).expect("nodes should exist");
        graph.insert_edge(5, 6, ()).expect("nodes should exist");
        graph.insert_edge(3, 2, ()).expect("nodes should exist");
        graph.insert_edge(5, 1, ()).expect("nodes should exist");

        let expected_parents = HashMap::from([(1, 0), (2, 0), (3, 1), (4, 1), (5, 2), (6, 2)]);
        for (parent_edge, node) in pfs(&graph, 0) {
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
    fn pfs_ungraph() {
        let mut graph = UnMapGraph::with_capacity(5, 5);
        graph.put_node("a", ());
        graph.put_node("b", ());
        graph.put_node("c", ());
        graph.put_node("d", ());
        graph.put_node("e", ());
        graph.insert_edge("a", "b", ()).expect("nodes should exist");
        graph.insert_edge("c", "a", ()).expect("nodes should exist");
        graph.insert_edge("b", "d", ()).expect("nodes should exist");
        graph.insert_edge("d", "e", ()).expect("nodes should exist");
        graph.insert_edge("e", "c", ()).expect("nodes should exist");

        let expected_parents = HashMap::from([("b", "a"), ("c", "a"), ("d", "b"), ("e", "c")]);
        for (parent_edge, node) in pfs(&graph, "a") {
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

    #[test]
    fn pfs_even_edges() {
        let mut graph = UnMapGraph::with_capacity(5, 5);
        graph.put_node("a", ());
        graph.put_node("b", ());
        graph.put_node("c", ());
        graph.put_node("d", ());
        graph.put_node("e", ());
        graph.insert_edge("a", "b", 1).expect("nodes should exist");
        graph.insert_edge("c", "a", 2).expect("nodes should exist");
        graph.insert_edge("b", "d", 3).expect("nodes should exist");
        graph.insert_edge("e", "c", 4).expect("nodes should exist");
        graph.insert_edge("d", "e", 5).expect("nodes should exist");

        let expected_parents = HashMap::from([("c", "a"), ("e", "c")]);
        for (parent_edge, node) in pfs_where(&graph, "a", |&edge, _| *edge % 2 == 0) {
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
