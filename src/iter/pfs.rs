use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;
use crate::iter::traits::{Path, Traversal, Tree};
use priority_queue::PriorityQueue;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn pfs<'a, G, P, A>(
    graph: &'a G,
    start: G::NId,
    start_priority: P,
    priority_type: PriorityType,
    accumulator: A,
) -> Pfs<'a, G, P, A, impl Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
{
    Pfs::new(
        graph,
        start,
        start_priority,
        priority_type,
        accumulator,
        |_, _| true,
    )
}

pub fn pfs_where<'a, G, P, A, F>(
    graph: &'a G,
    start: G::NId,
    start_priority: P,
    priority_type: PriorityType,
    accumulator: A,
    condition: F,
) -> Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    Pfs::new(
        graph,
        start,
        start_priority,
        priority_type,
        accumulator,
        condition,
    )
}

// TODO: make pq hold PriorityType<P> and let user pass in Max or Min for priority ordering
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PriorityType {
    Max,
    Min,
}

#[derive(Eq, PartialEq, Clone)]
struct Priority<P> {
    value: P,
    ptype: PriorityType,
}

impl<P: Ord> Ord for Priority<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.value.cmp(&other.value);
        match self.ptype {
            PriorityType::Max => ordering,
            PriorityType::Min => ordering.reverse(),
        }
    }
}

impl<P: Ord> PartialOrd for Priority<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P> Priority<P> {
    fn new(value: P, ptype: PriorityType) -> Self {
        Self { value, ptype }
    }
}

pub struct Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    graph: &'a G,
    pq: PriorityQueue<G::NId, Priority<P>>,
    parent: HashMap<G::NId, Option<G::EId>>,
    priority: HashMap<G::NId, P>,
    accumulator: A,
    condition: F,
    priority_type: PriorityType,
}

impl<'a, G, P, A, F> Iterator for Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    type Item = (
        Option<Edge<'a, G::NId, G::EId, G::E>>,
        Node<'a, G::NId, G::N>,
        P,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let (node_id, priority) = self.pq.pop()?;
        if !self.parent.contains_key(&node_id) {
            self.parent.insert(node_id, None);
        }
        self.priority.insert(node_id, priority.value.clone());

        for (edge, node) in self.graph.adj(node_id)? {
            if (self.condition)(&edge, &node) {
                let next_id = node.id();

                if !self.priority.contains_key(&next_id) {
                    let next_priority = Priority::new(
                        (self.accumulator)(priority.value.clone(), &edge, &node),
                        self.priority_type,
                    );
                    let old_priority = self.pq.push_increase(next_id, next_priority.clone());

                    match old_priority {
                        // update parent if priority is increased
                        Some(old_cost) if old_cost.value == next_priority.value => {}
                        _ => {
                            self.parent.insert(next_id, Some(edge.id()));
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
        Some((parent_edge_opt, node, priority.value))
    }
}

impl<'a, G, P, A, F> Tree<'a, G> for Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        let &edge_id = self.parent.get(&id)?.as_ref()?;
        self.graph.edge(edge_id)
    }

    fn path_to(&self, target: G::NId) -> Option<Path<'a, G>> {
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

impl<'a, G, P, A, F> Traversal<'a, G> for Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    type StepItem = Self::Item;

    fn is_visited(&self, node_id: G::NId) -> bool {
        self.parent.contains_key(&node_id)
    }

    fn current_node(&self) -> Option<Node<'a, G::NId, G::N>> {
        self.graph.node(*self.pq.peek()?.0)
    }

    fn find_path_to(&mut self, target: G::NId) -> Option<Path<'a, G>> {
        while !self.priority.contains_key(&target) {
            self.next()?;
        }
        self.path_to(target)
    }
}

impl<'a, G, P, A, F> Pfs<'a, G, P, A, F>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    fn new(
        graph: &'a G,
        start: G::NId,
        start_priority: P,
        priority_type: PriorityType,
        accumulator: A,
        condition: F,
    ) -> Self {
        Pfs {
            graph,
            pq: PriorityQueue::from(vec![(start, Priority::new(start_priority, priority_type))]),
            parent: HashMap::new(),
            priority: HashMap::new(),
            accumulator: accumulator,
            condition: condition,
            priority_type: priority_type,
        }
    }

    pub fn priority(&self, id: G::NId) -> Option<&P> {
        self.priority.get(&id)
    }
}

/*
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
}*/
