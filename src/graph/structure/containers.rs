use crate::graph::structure::edge::Edge;
use crate::graph::structure::node::Node;
use std::hash::Hash;
use std::iter::Iterator;

pub trait Keyed {}
pub trait Ordinal {}

pub trait NodeContainer<'a> {
    type NId: Eq + Hash + Copy;
    type N: 'a;

    type NodeIterator: Iterator<Item = Node<'a, Self::NId, Self::N>>;

    fn nodes(&'a self) -> Self::NodeIterator;

    fn len(&self) -> usize;

    fn contains_node(&'a self, id: Self::NId) -> bool;
    fn node(&'a self, id: Self::NId) -> Option<Node<Self::NId, Self::N>>;
    fn node_data(&'a self, id: Self::NId) -> Option<&Self::N>;
    fn node_data_mut(&mut self, id: Self::NId) -> Option<&mut Self::N>;

    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N>;
}

pub trait OrdinalNodeContainer<'a>: NodeContainer<'a> + Ordinal {
    fn insert_node(&mut self, node: Self::N) -> Self::NId;
}

pub trait KeyedNodeContainer<'a>: NodeContainer<'a> + Keyed {
    fn put_node(&'a mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;
}

use std::collections::HashMap;
use std::collections::hash_map::Iter;

pub struct NodeMap<Id, N> {
    nodes: HashMap<Id, N>
}

impl<'a, Id, N> NodeContainer<'a> for NodeMap<Id, N>
where
    Id: 'a + Eq + Hash + Copy,
    N: 'a,
{
    type NId = Id;
    type N = N;

    type NodeIterator = NodeIterator<'a, Id, N>;

    fn nodes(&self) -> NodeIterator<Id, N> {
        NodeIterator {
            inner: self.nodes.iter(),
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn contains_node(&self, id: Id) -> bool {
        self.nodes.contains_key(&id)
    }

    fn node(&self, id: Id) -> Option<Node<Id, N>> {
        self.nodes.get(&id).map(|n| Node::new(id, n))
    }

    fn node_data(&self, id: Id) -> Option<&N> {
        self.nodes.get(&id)
    }

    fn node_data_mut(&mut self, id: Id) -> Option<&mut N> {
        self.nodes.get_mut(&id)
    }

    fn remove_node(&mut self, id: Id) -> Option<N> {
        self.nodes.remove(&id)
    }
}

impl<Id, N> Keyed for NodeMap<Id, N> {}

impl<'a, Id, N> KeyedNodeContainer<'a> for NodeMap<Id, N>
where
    Id: 'a + Eq + Hash + Copy,
    N: 'a,
{
    fn put_node(&mut self, id: Id, node: N) -> Option<N> {
        let previous = self.remove_node(id);
        self.nodes.insert(id, node);

        previous
    }
}

pub struct NodeIterator<'a, Id, N> {
    inner: Iter<'a, Id, N>,
}

impl<'a, Id: Copy + Eq + Hash, N: 'a> Iterator for NodeIterator<'a, Id, N> 
{
    type Item = Node<'a, Id, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, n)| Node::new(*id, n))
    }
}

pub trait EdgeContainer<'a> {
    type NId: Eq + Hash + Copy;
    type E: 'a;
    type EId: Eq + Hash + Copy;

    type EdgeIterator: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>;

    fn edges(&'a self) -> Self::EdgeIterator;

    fn len(&self) -> usize;

    fn edge(&'a self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn edge_data(&'a self, id: Self::EId) -> Option<&Self::E>;
    fn edge_data_mut(&mut self, id: Self::EId) -> Option<&mut Self::E>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId>;
    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;
}

struct InternalEdge<Id, E> {
    u: Id,
    v: Id,
    e: E,
}
pub struct EdgeStableVec<NId, E> {
    edges: Vec<Option<InternalEdge<NId, E>>>,
    edges_len: usize,
}

impl<'a, NId, E> EdgeContainer<'a> for EdgeStableVec<NId, E>
where
    NId: 'a + Eq + Hash + Copy,
    E: 'a,
{
    type NId = NId;
    type E = E;
    type EId = usize;

    type EdgeIterator = EdgeIterator<'a, NId, E>;

    fn edges(&'a self) -> Self::EdgeIterator {
        EdgeIterator {
            inner: self.edges.iter().enumerate(),
        }
    }

    fn len(&self) -> usize {
        self.edges_len
    }

    fn edge(&'a self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        match self.edges.get(id) {
            Some(Some(edge)) => Some(Edge::new(id, edge.u, edge.v, &edge.e)),
            _ => None,
        }
    }

    fn edge_data(&self, id: usize) -> Option<&E> {
        Some(&self.edges.get(id)?.as_ref()?.e)
    }

    fn edge_data_mut(&mut self, id: usize) -> Option<&mut E> {
        Some(&mut self.edges.get_mut(id)?.as_mut()?.e)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId> {
        let id = self.edges.len();
        self.edges.push(Some(InternalEdge { u, v, e: edge }));
        self.edges_len += 1;
        Some(id)
    }

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E> {
        let internal_edge = self.edges.remove(id)?;
        self.edges_len -= 1;
        Some(internal_edge.e)
    }
}

use std::iter;
use std::slice;
pub struct EdgeIterator<'a, NId, E> {
    inner: iter::Enumerate<slice::Iter<'a, Option<InternalEdge<NId, E>>>>,
}

impl<'a, NId, E> Iterator for EdgeIterator<'a, NId, E>
where
    NId: 'a + Eq + Hash + Copy,
    E: 'a,
{
    type Item = Edge<'a, NId, usize, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let edge = opt.as_ref().unwrap();
                return Some(Edge::new(id, edge.u, edge.v, &edge.e));
            }
        }
    }
}

pub trait AdjContainer<'a> {
    type NId: Eq + Hash + Copy;
    type EId: Eq + Hash + Copy;

    type AdjIterator: Iterator<Item = (Self::EId, Self::NId)>;

    fn adj(&'a self, u: Self::NId) -> Option<Self::AdjIterator>;
    fn between(&'a self, u: Self::NId, v: Self::NId) -> Option<Self::EId>;
    fn degree(&'a self, u: Self::NId) -> usize;

    fn insert_node(&mut self, u: Self::NId);
    fn remove_node(&mut self, u: Self::NId);
    fn clear_node(&mut self, u: Self::NId);

    fn contains_edge(&'a self, u: Self::NId, v: Self::NId) -> bool;
    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
}

pub trait MultiAdjContainer<'a>: AdjContainer<'a> {
    type MultiEdgeIterator: Iterator<Item = (Self::NId, Self::EId)>;

    fn between_multi(&'a self, u: Self::NId, v: Self::NId) -> Option<Self::MultiEdgeIterator>;
}

pub trait OrdinalAdjContainer<'a>: AdjContainer<'a> + Ordinal {}
pub trait KeyedAdjContainer<'a>: AdjContainer<'a> + Keyed {}

pub struct AdjMap<NId, EId> {
    adj: HashMap<NId, HashMap<NId, EId>>,
}

impl<'a, NId, EId> AdjContainer<'a> for AdjMap<NId, EId>
where
    NId: 'a + Eq + Hash + Copy,
    EId: 'a + Eq + Hash + Copy,
{
    type NId = NId;
    type EId = EId;

    type AdjIterator = AdjIterator<'a, NId, EId>;


    fn adj(&'a self, u: Self::NId) -> Option<Self::AdjIterator> {
        Some(AdjIterator{
            inner: self.adj.get(&u)?.iter()
        })
    }

    fn between(&'a self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        Some(self.adj.get(&u)?.get(&v)?.clone())
    }

    fn degree(&'a self, u: Self::NId) -> usize {
        self.adj.get(&u).map_or(0, |adj_map| adj_map.len())
    }

    fn insert_node(&mut self, u: Self::NId) {
        self.adj.insert(u, HashMap::new());
    }

    fn remove_node(&mut self, u: Self::NId) {
        self.adj.remove(&u);
    }

    fn clear_node(&mut self, u: Self::NId) {
        self.adj.get_mut(&u).unwrap().clear();
    }

    fn contains_edge(&'a self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.get(&u).is_some() && self.adj[&u].contains_key(&v)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.get_mut(&u).unwrap().insert(v, edge_id);
    }

    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, _edge_id: Self::EId) {
        self.adj
            .get_mut(&u)
            .unwrap()
            .remove(&v);
    }
}

impl<NId, EId> Keyed for AdjMap<NId, EId> {}

impl<'a, NId, EId> KeyedAdjContainer<'a> for AdjMap<NId, EId>
where
    NId: 'a + Eq + Hash + Copy,
    EId: 'a + Eq + Hash + Copy,
{}

pub struct AdjIterator<'a, NId, EId> {
    inner: Iter<'a, NId, EId>,
}

impl<'a, NId, EId> Iterator for AdjIterator<'a, NId, EId>
where
    NId: Eq + Hash + Copy,
    EId: Eq + Hash + Copy,
{
    type Item = (NId, EId);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&v, &id)| {
            (id, v)
        })
    }
}

use std::marker::PhantomData;

// TODO: move the 'a and complex where statement to the impl instead of the struct itself
pub struct UnGraph<NC, EC, AC> {
    nodes: NC,
    edges: EC,
    adj: AC,
}

impl<'a, NC, EC, AC> Graph<'a> for UnGraph<NC, EC, AC> 
where
    NC: NodeContainer<'a>,
    EC: EdgeContainer<'a, NId=NC::NId>,
    AC: AdjContainer<'a, NId=NC::NId, EId=EC::EId>,
{
    type N = NC::N;
    type NId = NC::NId;
    type E = EC::E;
    type EId = EC::EId;

    type NodeIterator = NC::NodeIterator;
    type EdgeIterator = EC::EdgeIterator;
    type AdjIterator = DGAdjIterator<'a, NC, EC, AC>;

    fn len(&self) -> (usize, usize) {
        (self.nodes.len(), self.edges.len())
    }

    fn contains_node(&self, id: Self::NId) -> bool {
        self.nodes.contains_node(id)
    }

    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>> {
        self.nodes.node(id)
    }

    fn node_data(&self, id: Self::NId) -> Option<&Self::N> {
        self.nodes.node_data(id)
    }

    fn node_data_mut(&mut self, id: Self::NId) -> Option<&mut Self::N> {
        self.nodes.node_data_mut(id)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.adj.degree(u)
    }

    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N> {
        self.clear_node(id);
        self.adj.remove_node(id);
        self.nodes.remove_node(id)
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<()> {
        if !self.contains_node(u) {
            return None;
        }
        let adj_ids: Vec<_> = self.adj(u).collect();
        self.adj.clear_node(u);
        for (edge_id, v) in adj_ids {
            self.adj.remove_edge(v, u, edge_id);
            self.edges.remove_edge(edge_id).expect("Edge should be present");
        }
        Some(())
    }

    fn contains_edge(&'a self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.contains_edge(u, v)
    }

    fn edge(&'a self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        self.edges.edge(id)
    }

    fn between(&'a self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        let edge_id = self.adj.between(u, v)?;
        self.edges.edge(edge_id)
    }

    fn edge_data(&'a self, id: Self::EId) -> Option<&Self::E> {
        self.edges.edge_data(id)
    }

    fn edge_data_mut(&mut self, id: Self::EId) -> Option<&mut Self::E> {
        self.edges.edge_data_mut(id)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId> {
        if !self.contains_node(u) || !self.contains_node(v) {
            return None;
        }

        let edge_id = self.edges.insert_edge(u, v, edge)?;
        self.adj.insert_edge(u, v, edge_id);
        self.adj.insert_edge(v, u, edge_id);
        Some(edge_id)
    }

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E> {
        let edge = self.edge(id)?;
        let (u, v) = (edge.u(), edge.v());
        self.adj.remove_edge(u, v, id);
        self.adj.remove_edge(v, u, id);
        self.edges.remove_edge(id);
    }
}

pub struct DGAdjIterator<'a, NC, EC, AC> {
    graph: &'a UnGraph<NC, EC, AC>,
    inner: AC::AdjIterator,
}

impl<'a, NC, EC, AC> Iterator for DGAdjIterator<'a, NC, EC, AC> {
    type Item = (
        Edge<'a, NC::NId, EC::EId, EC::E>,
        Node<'a, NC::NId, NC::N>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&id, &v)| {
            (self.graph.edges.edge(id), self.graph.nodes.node(v))
        })
    }
}

// impl UnGraph where AC: Ordinal, NC: Ordinal
// impl UnGraph where AC: Keyed

pub struct DiGraph<'a, NC, EC, AC>
where
    NC: NodeContainer<'a>,
    EC: EdgeContainer<'a, NId=NC::NId>,
    AC: AdjContainer<'a, NId=NC::NId, EId=EC::EId>,
{
    nodes: NC,
    edges: EC,
    out_adj: AC,
    in_adj: AC,

    phantom: PhantomData<&'a NC>, // this is because the struct doesn't store anything directly
                                  // using lifetime 'a, but we need the lifetime to use in the
                                  // where clause
}

type BlahGraph<'a> = DiGraph<'a, NodeMap<usize, ()>, EdgeStableVec<usize, ()>, AdjMap<usize, usize>>;
