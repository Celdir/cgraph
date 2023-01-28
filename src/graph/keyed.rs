use crate::graph::edge::{Edge, EdgeMut};
use crate::graph::node::{Node, NodeMut};
use crate::graph::traits::{
    DirectedGraph, Graph, KeyedGraph, OrdinalGraph, UndirectedGraph, WithCapacity,
};

use std::hash::Hash;

use bimap::BiMap;

pub struct Keyed<G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    graph: G,
    keys: BiMap<Id, G::NId>,
}

impl<G, Id> Graph for Keyed<G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    type N = G::N;
    type NId = Id;
    type E = G::E;
    type EId = G::EId;

    type NodeIterator<'a> = NodeIterator<'a, G, Id> where Self: 'a;
    type NodeMutIterator<'a> = NodeMutIterator<'a, G, Id> where Self: 'a;

    type EdgeIterator<'a> = EdgeIterator<'a, G, Id> where Self: 'a;
    type EdgeMutIterator<'a> = EdgeMutIterator<'a, G, Id> where Self: 'a;

    type AdjIterator<'a> = AdjIterator<'a, G, Id> where Self: 'a;
    type AdjMutIterator<'a> = AdjMutIterator<'a, G, Id> where Self: 'a;

    fn len(&self) -> (usize, usize) {
        self.graph.len()
    }

    fn contains_node(&self, id: Self::NId) -> bool {
        self.keys.contains_left(&id)
    }

    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>> {
        let &key = self.keys.get_by_left(&id)?;
        let inner_node = self.graph.node(key)?;
        Some(map_node(&self.keys, inner_node))
    }

    fn node_mut(&mut self, id: Self::NId) -> Option<NodeMut<Self::NId, Self::N>> {
        let &key = self.keys.get_by_left(&id)?;
        let inner_node_mut = self.graph.node_mut(key)?;
        Some(map_node_mut(&self.keys, inner_node_mut))
    }

    fn degree(&self, u: Self::NId) -> usize {
        match self.keys.get_by_left(&u) {
            Some(&key) => self.graph.degree(key),
            _ => 0,
        }
    }

    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N> {
        let &key = self.keys.get_by_left(&id)?;
        self.keys.remove_by_left(&id);
        self.graph.remove_node(key)
    }

    fn clear_node(&mut self, id: Self::NId) -> Option<()> {
        let &key = self.keys.get_by_left(&id)?;
        self.graph.clear_node(key)
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        let key_pair = (self.keys.get_by_left(&u), self.keys.get_by_left(&v));
        match key_pair {
            (Some(&u_key), Some(&v_key)) => self.graph.contains_edge(u_key, v_key),
            _ => false,
        }
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        let inner_edge = self.graph.edge(id)?;
        Some(map_edge(&self.keys, inner_edge))
    }

    fn edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        let inner_edge = self.graph.edge_mut(id)?;
        Some(map_edge_mut(&self.keys, inner_edge))
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        let key_pair = (self.keys.get_by_left(&u), self.keys.get_by_left(&v));
        match key_pair {
            (Some(&u_key), Some(&v_key)) => {
                Some(map_edge(&self.keys, self.graph.between(u_key, v_key)?))
            }
            _ => None,
        }
    }

    fn between_mut(
        &mut self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        let key_pair = (self.keys.get_by_left(&u), self.keys.get_by_left(&v));
        match key_pair {
            (Some(&u_key), Some(&v_key)) => Some(map_edge_mut(
                &self.keys,
                self.graph.between_mut(u_key, v_key)?,
            )),
            _ => None,
        }
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId> {
        let (&u_key, &v_key) = (self.keys.get_by_left(&u)?, self.keys.get_by_left(&v)?);
        self.graph.insert_edge(u_key, v_key, edge)
    }

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E> {
        self.graph.remove_edge(id)
    }

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a> {
        NodeIterator {
            keys: &self.keys,
            inner: self.graph.nodes(),
        }
    }

    fn nodes_mut<'a>(&'a mut self) -> Self::NodeMutIterator<'a> {
        NodeMutIterator {
            keys: &self.keys,
            inner: self.graph.nodes_mut(),
        }
    }

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a> {
        EdgeIterator {
            keys: &self.keys,
            inner: self.graph.edges(),
        }
    }

    fn edges_mut<'a>(&'a mut self) -> Self::EdgeMutIterator<'a> {
        EdgeMutIterator {
            keys: &self.keys,
            inner: self.graph.edges_mut(),
        }
    }

    // Returns out edges for directed graph or all edges for undirected
    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        let &key = self.keys.get_by_left(&u)?;
        Some(AdjIterator {
            keys: &self.keys,
            inner: self.graph.adj(key)?,
        })
    }

    fn adj_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        let &key = self.keys.get_by_left(&u)?;
        Some(AdjMutIterator {
            keys: &self.keys,
            inner: self.graph.adj_mut(key)?,
        })
    }
}

impl<G, Id> WithCapacity for Keyed<G, Id>
where
    G: Graph + WithCapacity,
    Id: Eq + Hash + Copy,
{
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            graph: G::with_capacity(node_capacity, edge_capacity),
            keys: BiMap::new(),
        }
    }
}

impl<G, Id> KeyedGraph for Keyed<G, Id>
where
    G: OrdinalGraph,
    Id: Eq + Hash + Copy,
{
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N> {
        let key = self.graph.insert_node(node);
        self.keys.insert(id, key);
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
    G: DirectedGraph,
    Id: Eq + Hash + Copy,
{
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        let &key = self.keys.get_by_left(&u)?;
        Some(AdjIterator {
            keys: &self.keys,
            inner: self.graph.out_edges(key)?,
        })
    }

    fn out_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        let &key = self.keys.get_by_left(&u)?;
        Some(AdjMutIterator {
            keys: &self.keys,
            inner: self.graph.out_edges_mut(key)?,
        })
    }

    fn out_degree(&self, u: Self::NId) -> usize {
        let key_opt = self.keys.get_by_left(&u);
        match key_opt {
            Some(&key) => self.graph.out_degree(key),
            _ => 0,
        }
    }

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        let &key = self.keys.get_by_left(&u)?;
        Some(AdjIterator {
            keys: &self.keys,
            inner: self.graph.in_edges(key)?,
        })
    }

    fn in_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        let &key = self.keys.get_by_left(&u)?;
        Some(AdjMutIterator {
            keys: &self.keys,
            inner: self.graph.in_edges_mut(key)?,
        })
    }

    fn in_degree(&self, u: Self::NId) -> usize {
        let key_opt = self.keys.get_by_left(&u);
        match key_opt {
            Some(&key) => self.graph.in_degree(key),
            _ => 0,
        }
    }

    fn reverse_edge(&mut self, id: Self::EId) -> Option<()> {
        self.graph.reverse_edge(id)
    }
}

impl<G, Id> UndirectedGraph for Keyed<G, Id>
where
    G: UndirectedGraph,
    Id: Eq + Hash + Copy,
{
}

pub struct NodeIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy,
{
    keys: &'a BiMap<Id, G::NId>,
    inner: G::NodeIterator<'a>,
}

impl<'a, G, Id> Iterator for NodeIterator<'a, G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    type Item = Node<'a, Id, G::N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|node| map_node(&self.keys, node))
    }
}

pub struct NodeMutIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy,
{
    keys: &'a BiMap<Id, G::NId>,
    inner: G::NodeMutIterator<'a>,
}

impl<'a, G, Id> Iterator for NodeMutIterator<'a, G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    type Item = NodeMut<'a, Id, G::N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|node| map_node_mut(&self.keys, node))
    }
}

pub struct EdgeIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy,
{
    keys: &'a BiMap<Id, G::NId>,
    inner: G::EdgeIterator<'a>,
}

impl<'a, G, Id> Iterator for EdgeIterator<'a, G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    type Item = Edge<'a, Id, G::EId, G::E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|node| map_edge(&self.keys, node))
    }
}

pub struct EdgeMutIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy,
{
    keys: &'a BiMap<Id, G::NId>,
    inner: G::EdgeMutIterator<'a>,
}

impl<'a, G, Id> Iterator for EdgeMutIterator<'a, G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    type Item = EdgeMut<'a, Id, G::EId, G::E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|node| map_edge_mut(&self.keys, node))
    }
}

pub struct AdjIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy,
{
    keys: &'a BiMap<Id, G::NId>,
    inner: G::AdjIterator<'a>,
}

impl<'a, G, Id> Iterator for AdjIterator<'a, G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    type Item = (Edge<'a, Id, G::EId, G::E>, Node<'a, Id, G::N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|adj| map_adj(&self.keys, adj))
    }
}

pub struct AdjMutIterator<'a, G, Id>
where
    G: 'a + Graph,
    Id: Eq + Hash + Copy,
{
    keys: &'a BiMap<Id, G::NId>,
    inner: G::AdjMutIterator<'a>,
}

impl<'a, G, Id> Iterator for AdjMutIterator<'a, G, Id>
where
    G: Graph,
    Id: Eq + Hash + Copy,
{
    type Item = (EdgeMut<'a, Id, G::EId, G::E>, NodeMut<'a, Id, G::N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|adj| map_adj_mut(&self.keys, adj))
    }
}

fn map_node<'a, L, R, N>(keys: &'a BiMap<L, R>, node: Node<'a, R, N>) -> Node<'a, L, N>
where
    L: Eq + Hash + Copy,
    R: Eq + Hash + Copy,
{
    let &id = keys
        .get_by_right(&node.id())
        .expect("Inner node id should be mapped to outer node id");
    Node::new(id, node.data())
}

fn map_node_mut<'a, L, R, N>(keys: &'a BiMap<L, R>, node: NodeMut<'a, R, N>) -> NodeMut<'a, L, N>
where
    L: Eq + Hash + Copy,
    R: Eq + Hash + Copy,
{
    let &id = keys
        .get_by_right(&node.id())
        .expect("Inner node id should be mapped to outer node id");
    NodeMut::new(id, node.into_data())
}

fn map_edge<'a, L, R, EId, E>(
    keys: &'a BiMap<L, R>,
    edge: Edge<'a, R, EId, E>,
) -> Edge<'a, L, EId, E>
where
    L: Eq + Hash + Copy,
    R: Eq + Hash + Copy,
    EId: 'a + Eq + Copy,
{
    let &u = keys
        .get_by_right(&edge.u())
        .expect("Inner node id should be mapped to outer node id");
    let &v = keys
        .get_by_right(&edge.v())
        .expect("Inner node id should be mapped to outer node id");
    Edge::new(edge.id(), u, v, edge.data())
}

fn map_edge_mut<'a, L, R, EId, E>(
    keys: &'a BiMap<L, R>,
    edge: EdgeMut<'a, R, EId, E>,
) -> EdgeMut<'a, L, EId, E>
where
    L: Eq + Hash + Copy,
    R: Eq + Hash + Copy,
    EId: 'a + Eq + Copy,
{
    let &u = keys
        .get_by_right(&edge.u())
        .expect("Inner node id should be mapped to outer node id");
    let &v = keys
        .get_by_right(&edge.v())
        .expect("Inner node id should be mapped to outer node id");
    EdgeMut::new(edge.id(), u, v, edge.into_data())
}

fn map_adj<'a, L, R, N, EId, E>(
    keys: &'a BiMap<L, R>,
    adj: (Edge<'a, R, EId, E>, Node<'a, R, N>),
) -> (Edge<'a, L, EId, E>, Node<'a, L, N>)
where
    L: Eq + Hash + Copy,
    R: Eq + Hash + Copy,
    EId: 'a + Eq + Copy,
{
    (map_edge(keys, adj.0), map_node(keys, adj.1))
}

fn map_adj_mut<'a, L, R, N, EId, E>(
    keys: &'a BiMap<L, R>,
    adj: (EdgeMut<'a, R, EId, E>, NodeMut<'a, R, N>),
) -> (EdgeMut<'a, L, EId, E>, NodeMut<'a, L, N>)
where
    L: Eq + Hash + Copy,
    R: Eq + Hash + Copy,
    EId: 'a + Eq + Copy,
{
    (map_edge_mut(keys, adj.0), map_node_mut(keys, adj.1))
}
