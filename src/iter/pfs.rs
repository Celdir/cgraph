use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;
use crate::graph::types::NodeHashMap;
use crate::iter::traits::{Path, PathTree, Traversal, Tree, WeightedPathTree};

use dary_heap::DaryHeap;
use std::cmp::Ordering;

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

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PriorityType {
    Max,
    Min,
}

#[derive(Eq, PartialEq, Clone)]
enum Priority<P> {
    Max(P),
    Min(P),
}

#[derive(Eq, PartialEq, Clone)]
struct PQItem<NId, EId, P> {
    priority: Priority<P>,
    node: NId,
    edge: Option<EId>,
}

impl<NId: Eq, EId: Eq, P: Ord> Ord for PQItem<NId, EId, P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<NId: Eq, EId: Eq, P: Ord> PartialOrd for PQItem<NId, EId, P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Ord> Ord for Priority<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Priority::Max(val) => val.cmp(other.val()),
            Priority::Min(val) => val.cmp(other.val()).reverse(),
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
        match ptype {
            PriorityType::Max => Priority::Max(value),
            PriorityType::Min => Priority::Min(value),
        }
    }

    fn val(&self) -> &P {
        match &self {
            Priority::Max(val) => val,
            Priority::Min(val) => val,
        }
    }

    fn into_val(self) -> P {
        match self {
            Priority::Max(val) => val,
            Priority::Min(val) => val,
        }
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
    pq: DaryHeap<PQItem<G::NId, G::EId, P>, 4>,
    tree: PathTree<'a, G>,
    priority: NodeHashMap<G, P>,
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
        let item = self.pq.pop()?;
        if self.priority.contains_key(&item.node) {
            return self.next();
        }
        self.priority.insert(item.node, item.priority.val().clone());
        self.tree.insert_parent(item.node, item.edge);

        for (edge, node) in self.graph.adj(item.node)? {
            if (self.condition)(&edge, &node) {
                let next_id = node.id();

                if !self.priority.contains_key(&next_id) {
                    let next_priority = Priority::new(
                        (self.accumulator)(item.priority.val().clone(), &edge, &node),
                        self.priority_type,
                    );
                    self.pq.push(PQItem {
                        node: next_id,
                        edge: Some(edge.id()),
                        priority: next_priority,
                    });
                }
            }
        }

        let node = self.graph.node(item.node).unwrap();

        Some((self.parent_edge(item.node), node, item.priority.into_val()))
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
        if !self.priority.contains_key(&id) {
            return None;
        }
        self.tree.parent_edge(id)
    }

    fn path_to(&self, target: G::NId) -> Option<Path<'a, G>> {
        if !self.priority.contains_key(&target) {
            return None;
        }
        self.tree.path_to(target)
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
        self.priority.contains_key(&node_id)
    }

    fn current_node(&self) -> Option<Node<'a, G::NId, G::N>> {
        self.graph.node(self.pq.peek()?.node)
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
            pq: DaryHeap::from([PQItem {
                node: start,
                edge: None,
                priority: Priority::new(start_priority, priority_type),
            }]),
            tree: PathTree::new(graph),
            priority: NodeHashMap::<G, P>::default(),
            accumulator: accumulator,
            condition: condition,
            priority_type: priority_type,
        }
    }

    pub fn priority(&self, id: G::NId) -> Option<&P> {
        self.priority.get(&id)
    }
}

impl<'a, G, P, A, F> From<Pfs<'a, G, P, A, F>> for WeightedPathTree<'a, G, P>
where
    G: Graph,
    P: Ord + Clone,
    A: Fn(P, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> P,
    F: Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
{
    fn from(pfs: Pfs<'a, G, P, A, F>) -> Self {
        let mut tree = WeightedPathTree::new(pfs.graph);
        for (edge, node, priority) in pfs {
            tree.insert_node(node.id(), edge.map(|e| e.id()), priority);
        }
        tree
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::graph::traits::{Graph, KeyedGraph, OrdinalGraph, WithCapacity, GraphMut};
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
