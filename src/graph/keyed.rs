use crate::graph::edge::{Edge, EdgeMut};
use crate::graph::errors::GraphError;
use crate::graph::node::{Node, NodeMut};
use crate::graph::traits::{
    DirectedGraph, DirectedGraphMut, Graph, GraphIter, GraphMut, KeyedGraph, OrdinalGraph,
    UndirectedGraph, WithCapacity,
};

use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;

use ahash::AHashMap;

pub struct Keyed<G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    graph: G,
    keys: AHashMap<Id, G::NId>,
    reverse_keys: AHashMap<G::NId, Id>,
}

impl<G, Id> Graph for Keyed<G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type N = G::N;
    type NId = Id;
    type E = G::E;
    type EId = G::EId;

    type AdjIterator<'a> = AdjIterator<'a, G, Id> where Self: 'a;
    type AdjIdsIterator<'a> = AdjIdsIterator<'a, G, Id> where Self: 'a;

    fn contains_node(&self, id: Self::NId) -> bool {
        self.keys.contains_key(&id)
    }

    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>> {
        let &key = self.keys.get(&id)?;
        let inner_node = self.graph.node(key)?;
        Some(map_node(&self.reverse_keys, inner_node))
    }

    fn degree(&self, u: Self::NId) -> usize {
        match self.keys.get(&u) {
            Some(&key) => self.graph.degree(key),
            _ => 0,
        }
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        let key_pair = (self.keys.get(&u), self.keys.get(&v));
        match key_pair {
            (Some(&u_key), Some(&v_key)) => self.graph.contains_edge(u_key, v_key),
            _ => false,
        }
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        let inner_edge = self.graph.edge(id)?;
        Some(map_edge(&self.reverse_keys, inner_edge))
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        let key_pair = (self.keys.get(&u), self.keys.get(&v));
        match key_pair {
            (Some(&u_key), Some(&v_key)) => Some(map_edge(
                &self.reverse_keys,
                self.graph.between(u_key, v_key)?,
            )),
            _ => None,
        }
    }

    // Returns out edges for directed graph or all edges for undirected
    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        let &key = self.keys.get(&u)?;
        Some(AdjIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.adj(key)?,
        })
    }

    fn adj_ids<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIdsIterator<'a>> {
        let &key = self.keys.get(&u)?;
        Some(AdjIdsIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.adj_ids(key)?,
        })
    }
}

impl<G, Id> GraphIter for Keyed<G, Id>
where
    G: GraphIter + OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type NodeIterator<'a> = NodeIterator<'a, G, Id> where Self: 'a;
    type EdgeIterator<'a> = EdgeIterator<'a, G, Id> where Self: 'a;

    fn len(&self) -> (usize, usize) {
        self.graph.len()
    }

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a> {
        NodeIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.nodes(),
        }
    }

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a> {
        EdgeIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.edges(),
        }
    }
}

impl<G, Id> GraphMut for Keyed<G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type NodeMutIterator<'a> = NodeMutIterator<'a, G, Id> where Self: 'a;
    type EdgeMutIterator<'a> = EdgeMutIterator<'a, G, Id> where Self: 'a;
    type AdjMutIterator<'a> = AdjMutIterator<'a, G, Id> where Self: 'a;

    fn node_mut(&mut self, id: Self::NId) -> Option<NodeMut<Self::NId, Self::N>> {
        let &key = self.keys.get(&id)?;
        let inner_node_mut = self.graph.node_mut(key)?;
        Some(map_node_mut(&self.reverse_keys, inner_node_mut))
    }

    fn remove_node(&mut self, id: Self::NId) -> Result<Self::N, GraphError> {
        let &key = self
            .keys
            .get(&id)
            .ok_or_else(|| GraphError::NodeNotFound(format!("{:?}", id)))?;
        self.keys.remove(&id);
        self.reverse_keys.remove(&key);
        self.graph.remove_node(key)
    }

    fn clear_node(&mut self, id: Self::NId) -> Result<(), GraphError> {
        let &key = self
            .keys
            .get(&id)
            .ok_or_else(|| GraphError::NodeNotFound(format!("{:?}", id)))?;
        self.graph.clear_node(key)
    }

    fn edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        let inner_edge = self.graph.edge_mut(id)?;
        Some(map_edge_mut(&self.reverse_keys, inner_edge))
    }

    fn between_mut(
        &mut self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        let key_pair = (self.keys.get(&u), self.keys.get(&v));
        match key_pair {
            (Some(&u_key), Some(&v_key)) => Some(map_edge_mut(
                &self.reverse_keys,
                self.graph.between_mut(u_key, v_key)?,
            )),
            _ => None,
        }
    }

    fn insert_edge(
        &mut self,
        u: Self::NId,
        v: Self::NId,
        edge: Self::E,
    ) -> Result<Self::EId, GraphError> {
        let (&u_key, &v_key) = (
            self.keys
                .get(&u)
                .ok_or_else(|| GraphError::NodeNotFound(format!("{:?}", u)))?,
            self.keys
                .get(&v)
                .ok_or_else(|| GraphError::NodeNotFound(format!("{:?}", v)))?,
        );
        self.graph.insert_edge(u_key, v_key, edge)
    }

    fn remove_edge(&mut self, id: Self::EId) -> Result<Self::E, GraphError> {
        self.graph.remove_edge(id)
    }

    fn nodes_mut<'a>(&'a mut self) -> Self::NodeMutIterator<'a> {
        NodeMutIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.nodes_mut(),
        }
    }

    fn edges_mut<'a>(&'a mut self) -> Self::EdgeMutIterator<'a> {
        EdgeMutIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.edges_mut(),
        }
    }

    fn adj_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        let &key = self.keys.get(&u)?;
        Some(AdjMutIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.adj_mut(key)?,
        })
    }
}

impl<G, Id> Default for Keyed<G, Id>
where
    G: OrdinalGraph + Default,
    Id: Eq + Hash + Copy + Debug,
{
    fn default() -> Self {
        Self {
            graph: G::default(),
            keys: AHashMap::new(),
            reverse_keys: AHashMap::new(),
        }
    }
}

impl<G, Id> WithCapacity for Keyed<G, Id>
where
    G: OrdinalGraph + WithCapacity,
    Id: Eq + Hash + Copy + Debug,
{
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            graph: G::with_capacity(node_capacity, edge_capacity),
            keys: AHashMap::new(),
            reverse_keys: AHashMap::new(),
        }
    }
}

impl<G, Id> KeyedGraph for Keyed<G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N> {
        if self.contains_node(id) {
            let node_data = self.node_mut(id).unwrap().into_data();
            return Some(std::mem::replace(node_data, node));
        }

        let key = self.graph.insert_node(node);
        self.keys.insert(id, key);
        self.reverse_keys.insert(key, id);
        None
    }

    fn from_keyed(
        nodes: Vec<(Self::NId, Self::N)>,
        edges: Vec<(Self::NId, Self::NId, Self::E)>,
    ) -> Self
    where
        Self: WithCapacity,
    {
        let mut g = Self::with_capacity(nodes.len(), edges.len());
        for (id, node) in nodes {
            g.put_node(id, node);
        }
        for (u, v, edge) in edges {
            g.insert_edge(u, v, edge)
                .expect("node ids in edge should refer to valid node");
        }

        g
    }
}

impl<G, Id> DirectedGraph for Keyed<G, Id>
where
    G: DirectedGraph + OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        let &key = self.keys.get(&u)?;
        Some(AdjIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.out_edges(key)?,
        })
    }

    fn out_degree(&self, u: Self::NId) -> usize {
        let key_opt = self.keys.get(&u);
        match key_opt {
            Some(&key) => self.graph.out_degree(key),
            _ => 0,
        }
    }

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        let &key = self.keys.get(&u)?;
        Some(AdjIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.in_edges(key)?,
        })
    }

    fn in_degree(&self, u: Self::NId) -> usize {
        let key_opt = self.keys.get(&u);
        match key_opt {
            Some(&key) => self.graph.in_degree(key),
            _ => 0,
        }
    }
}

impl<G, Id> DirectedGraphMut for Keyed<G, Id>
where
    G: DirectedGraphMut + OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    fn out_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        let &key = self.keys.get(&u)?;
        Some(AdjMutIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.out_edges_mut(key)?,
        })
    }

    fn in_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        let &key = self.keys.get(&u)?;
        Some(AdjMutIterator {
            reverse_keys: &self.reverse_keys,
            inner: self.graph.in_edges_mut(key)?,
        })
    }

    fn reverse_edge(&mut self, id: Self::EId) -> Result<(), GraphError> {
        self.graph.reverse_edge(id)
    }
}

impl<G, Id> UndirectedGraph for Keyed<G, Id>
where
    G: UndirectedGraph + OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
}

impl<G, Id> Keyed<G, Id>
where
    G: OrdinalGraph + Default,
    Id: Eq + Hash + Copy + Debug,
{
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct NodeIterator<'a, G, Id>
where
    G: 'a + GraphIter,
    Id: Eq + Hash + Copy + Debug,
{
    reverse_keys: &'a AHashMap<G::NId, Id>,
    inner: G::NodeIterator<'a>,
}

impl<'a, G, Id> Iterator for NodeIterator<'a, G, Id>
where
    G: GraphIter + OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type Item = Node<'a, Id, G::N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|node| map_node(&self.reverse_keys, node))
    }
}

pub struct NodeMutIterator<'a, G, Id>
where
    G: 'a + GraphMut,
    Id: Eq + Hash + Copy + Debug,
{
    reverse_keys: &'a AHashMap<G::NId, Id>,
    inner: G::NodeMutIterator<'a>,
}

impl<'a, G, Id> Iterator for NodeMutIterator<'a, G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type Item = NodeMut<'a, Id, G::N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|node| map_node_mut(&self.reverse_keys, node))
    }
}

pub struct EdgeIterator<'a, G, Id>
where
    G: 'a + GraphIter,
    Id: Eq + Hash + Copy + Debug,
{
    reverse_keys: &'a AHashMap<G::NId, Id>,
    inner: G::EdgeIterator<'a>,
}

impl<'a, G, Id> Iterator for EdgeIterator<'a, G, Id>
where
    G: GraphIter + OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type Item = Edge<'a, Id, G::EId, G::E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|node| map_edge(&self.reverse_keys, node))
    }
}

pub struct EdgeMutIterator<'a, G, Id>
where
    G: 'a + GraphMut,
    Id: Eq + Hash + Copy + Debug,
{
    reverse_keys: &'a AHashMap<G::NId, Id>,
    inner: G::EdgeMutIterator<'a>,
}

impl<'a, G, Id> Iterator for EdgeMutIterator<'a, G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type Item = EdgeMut<'a, Id, G::EId, G::E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|node| map_edge_mut(&self.reverse_keys, node))
    }
}

pub struct AdjIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy + Debug,
{
    reverse_keys: &'a AHashMap<G::NId, Id>,
    inner: G::AdjIterator<'a>,
}

impl<'a, G, Id> Iterator for AdjIterator<'a, G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type Item = (Edge<'a, Id, G::EId, G::E>, Node<'a, Id, G::N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|adj| map_adj(&self.reverse_keys, adj))
    }
}

pub struct AdjIdsIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy + Debug,
{
    reverse_keys: &'a AHashMap<G::NId, Id>,
    inner: G::AdjIdsIterator<'a>,
}

impl<'a, G, Id> Iterator for AdjIdsIterator<'a, G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type Item = (G::EId, Id);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|adj| {
            (
                adj.0,
                *self
                    .reverse_keys
                    .get(&adj.1)
                    .expect("Inner node id should be mapped to outer node id"),
            )
        })
    }
}

pub struct AdjMutIterator<'a, G, Id>
where
    G: 'a + GraphMut,
    Id: Eq + Hash + Copy + Debug,
{
    reverse_keys: &'a AHashMap<G::NId, Id>,
    inner: G::AdjMutIterator<'a>,
}

impl<'a, G, Id> Iterator for AdjMutIterator<'a, G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy + Debug,
{
    type Item = (EdgeMut<'a, Id, G::EId, G::E>, NodeMut<'a, Id, G::N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|adj| map_adj_mut(&self.reverse_keys, adj))
    }
}

fn map_node<'a, L, R, N>(reverse_keys: &'a AHashMap<R, L>, node: Node<'a, R, N>) -> Node<'a, L, N>
where
    L: Eq + Hash + Copy + Debug,
    R: Eq + Hash + Copy + Debug,
{
    let &id = reverse_keys
        .get(&node.id())
        .expect("Inner node id should be mapped to outer node id");
    Node::new(id, node.into_data())
}

fn map_node_mut<'a, L, R, N>(
    reverse_keys: &'a AHashMap<R, L>,
    node: NodeMut<'a, R, N>,
) -> NodeMut<'a, L, N>
where
    L: Eq + Hash + Copy + Debug,
    R: Eq + Hash + Copy + Debug,
{
    let &id = reverse_keys
        .get(&node.id())
        .expect("Inner node id should be mapped to outer node id");
    NodeMut::new(id, node.into_data())
}

fn map_edge<'a, L, R, EId, E>(
    reverse_keys: &'a AHashMap<R, L>,
    edge: Edge<'a, R, EId, E>,
) -> Edge<'a, L, EId, E>
where
    L: Eq + Hash + Copy + Debug,
    R: Eq + Hash + Copy + Debug,
    EId: 'a + Eq + Copy + Debug,
{
    let &u = reverse_keys
        .get(&edge.u())
        .expect("Inner node id should be mapped to outer node id");
    let &v = reverse_keys
        .get(&edge.v())
        .expect("Inner node id should be mapped to outer node id");
    Edge::new(edge.id(), u, v, edge.into_data())
}

fn map_edge_mut<'a, L, R, EId, E>(
    reverse_keys: &'a AHashMap<R, L>,
    edge: EdgeMut<'a, R, EId, E>,
) -> EdgeMut<'a, L, EId, E>
where
    L: Eq + Hash + Copy + Debug,
    R: Eq + Hash + Copy + Debug,
    EId: 'a + Eq + Copy + Debug,
{
    let &u = reverse_keys
        .get(&edge.u())
        .expect("Inner node id should be mapped to outer node id");
    let &v = reverse_keys
        .get(&edge.v())
        .expect("Inner node id should be mapped to outer node id");
    EdgeMut::new(edge.id(), u, v, edge.into_data())
}

fn map_adj<'a, L, R, N, EId, E>(
    reverse_keys: &'a AHashMap<R, L>,
    adj: (Edge<'a, R, EId, E>, Node<'a, R, N>),
) -> (Edge<'a, L, EId, E>, Node<'a, L, N>)
where
    L: Eq + Hash + Copy + Debug,
    R: Eq + Hash + Copy + Debug,
    EId: 'a + Eq + Copy + Debug,
{
    (map_edge(reverse_keys, adj.0), map_node(reverse_keys, adj.1))
}

fn map_adj_mut<'a, L, R, N, EId, E>(
    reverse_keys: &'a AHashMap<R, L>,
    adj: (EdgeMut<'a, R, EId, E>, NodeMut<'a, R, N>),
) -> (EdgeMut<'a, L, EId, E>, NodeMut<'a, L, N>)
where
    L: Eq + Hash + Copy + Debug,
    R: Eq + Hash + Copy + Debug,
    EId: 'a + Eq + Copy + Debug,
{
    (
        map_edge_mut(reverse_keys, adj.0),
        map_node_mut(reverse_keys, adj.1),
    )
}

#[cfg(test)]
mod tests {
    use crate::graph::keyed::Keyed;
    use crate::graph::traits::{DirectedGraph, Graph, GraphIter, GraphMut, KeyedGraph};
    use crate::graph::types::{DiFlatGraph, UnFlatGraph};

    #[test]
    fn un_keyed_puts_and_removes() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let mut graph = Keyed::<UnFlatGraph<(), i32>, &str>::new();
        graph.put_node("A", ());
        graph.put_node("B", ());
        graph.put_node("C", ());
        graph.put_node("D", ());
        graph.insert_edge("A", "B", 5).expect("nodes should exist");
        graph.insert_edge("A", "C", 2).expect("nodes should exist");
        graph.insert_edge("C", "D", 1).expect("nodes should exist");
        graph.insert_edge("B", "D", 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        let mut a_adj: Vec<_> = graph
            .adj("A")
            .expect("A should have adj edges")
            .map(|(_, node)| node.id())
            .collect();
        a_adj.sort();
        assert_eq!(a_adj, vec!["B", "C"]);

        // edges should be undirected and the same id each direction
        assert!(graph.between("A", "B").is_some());
        assert!(graph.between("B", "A").is_some());
        assert_eq!(
            graph.between("A", "B").unwrap().id(),
            graph.between("B", "A").unwrap().id()
        );

        // removes A and adjacent edges
        graph.remove_node("A").expect("node should exist");
        assert!(graph.node("A").is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.degree("B"), 1);
        assert_eq!(graph.degree("C"), 1);
        assert_eq!(graph.degree("D"), 2);

        // single deleted edge should be gone
        let edge_id = graph.between("D", "C").unwrap().id();
        graph.remove_edge(edge_id).unwrap();

        assert_eq!(graph.degree("C"), 0);
        assert_eq!(graph.degree("D"), 1);
    }

    #[test]
    fn di_keyed_puts_and_removes() {
        // A --5-> B
        // |       |
        // 2       1
        // v       v
        // C --1-> D
        let mut graph = Keyed::<DiFlatGraph<(), i32>, &str>::new();
        graph.put_node("A", ());
        graph.put_node("B", ());
        graph.put_node("C", ());
        graph.put_node("D", ());
        graph.insert_edge("A", "B", 5).expect("nodes should exist");
        graph.insert_edge("A", "C", 2).expect("nodes should exist");
        graph.insert_edge("C", "D", 1).expect("nodes should exist");
        graph.insert_edge("B", "D", 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        let mut a_out: Vec<_> = graph
            .out_edges("A")
            .expect("A should have adj edges")
            .map(|(_, node)| node.id())
            .collect();
        a_out.sort();
        assert_eq!(a_out, vec!["B", "C"]);

        // edges should be directed
        assert!(graph.between("A", "B").is_some());
        assert!(graph.between("B", "A").is_none());

        let mut d_in: Vec<_> = graph
            .in_edges("D")
            .expect("D should have in edges")
            .map(|(_, node)| node.id())
            .collect();
        d_in.sort();
        assert_eq!(d_in, vec!["B", "C"]);

        // removes A and adjacent edges
        graph.remove_node("A").expect("node should exist");
        assert!(graph.node("A").is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.out_degree("B"), 1);
        assert_eq!(graph.in_degree("B"), 0);
        assert_eq!(graph.out_degree("C"), 1);
        assert_eq!(graph.in_degree("C"), 0);
        assert_eq!(graph.out_degree("D"), 0);
        assert_eq!(graph.in_degree("D"), 2);

        assert!(graph.contains_edge("B", "D"));
        assert!(!graph.contains_edge("A", "B"));
    }

    #[test]
    fn un_keyed_iteration() {
        // A ----- B
        // |  \ /  |
        // |   X   |
        // |  / \  |
        // C ----- D
        let mut graph = Keyed::<UnFlatGraph<i32, i32>, &str>::builder()
            .nodes(vec![("A", 0), ("B", 0), ("C", 0), ("D", 0)])
            .edges(vec![
                ("A", "B", 2),
                ("B", "C", 2),
                ("C", "D", 2),
                ("A", "D", 2),
                ("A", "C", 2),
                ("D", "B", 2),
            ])
            .build();

        for mut node in graph.nodes_mut() {
            assert_eq!(*node, 0);
            *node = 10;
        }

        for node in graph.nodes() {
            assert_eq!(*node, 10);
        }

        for mut edge in graph.edges_mut() {
            assert_eq!(*edge, 2);
            *edge = 1;
        }

        for edge in graph.edges() {
            assert_eq!(*edge, 1);
        }

        let mut a_adj: Vec<_> = graph
            .adj("A")
            .expect("A should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        a_adj.sort();
        assert_eq!(a_adj, vec![("B", 1), ("C", 1), ("D", 1)]);

        let mut b_adj: Vec<_> = graph
            .adj("B")
            .expect("B should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        b_adj.sort();
        assert_eq!(b_adj, vec![("A", 1), ("C", 1), ("D", 1)]);

        let mut c_adj: Vec<_> = graph
            .adj("C")
            .expect("C should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        c_adj.sort();
        assert_eq!(c_adj, vec![("A", 1), ("B", 1), ("D", 1)]);

        let mut d_adj: Vec<_> = graph
            .adj("D")
            .expect("D should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        d_adj.sort();
        assert_eq!(d_adj, vec![("A", 1), ("B", 1), ("C", 1)]);

        *graph.node_mut("A").unwrap() = 3;
        assert_eq!(*graph.node("A").unwrap(), 3);

        *graph.edge_mut(0).unwrap() = 5;
        assert_eq!(*graph.edge(0).unwrap(), 5);
        *graph.edge_mut(0).unwrap() = 1;

        for (mut edge_mut, mut node_mut) in graph.adj_mut("A").unwrap() {
            assert_eq!(*edge_mut, 1);
            assert_eq!(*node_mut, 10);
            *edge_mut = 2;
            *node_mut = 0;
        }

        for (edge, node) in graph.adj("A").unwrap() {
            assert_eq!(*edge, 2);
            assert_eq!(*node, 0);
        }
    }
}
