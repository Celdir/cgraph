use crate::graph::structure::edge::Edge;
use crate::graph::structure::node::Node;
use std::hash::Hash;
use std::iter::Iterator;

pub trait Keyed {}
pub trait Ordinal {}

pub trait NodeContainer {
    type NId: Eq + Hash + Copy;
    type N;

    type NodeIterator<'a>: Iterator<Item = Node<'a, Self::NId, Self::N>>
    where
        Self: 'a;

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a>;

    fn len(&self) -> usize;

    fn contains_node(&self, id: Self::NId) -> bool;
    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>>;
    fn node_data(&self, id: Self::NId) -> Option<&Self::N>;
    fn node_data_mut(&mut self, id: Self::NId) -> Option<&mut Self::N>;

    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N>;
}

pub trait OrdinalNodeContainer: NodeContainer + Ordinal {
    fn insert_node(&mut self, node: Self::N) -> Self::NId;
}

pub trait KeyedNodeContainer: NodeContainer + Keyed {
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;
}

use std::collections::hash_map::Iter;
use std::collections::HashMap;

pub struct NodeMap<Id, N> {
    nodes: HashMap<Id, N>,
}

impl<Id, N> NodeContainer for NodeMap<Id, N>
where
    Id: Eq + Hash + Copy,
{
    type NId = Id;
    type N = N;

    type NodeIterator<'a> = NodeIterator<'a, Id, N> where Self: 'a;

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

impl<Id, N> KeyedNodeContainer for NodeMap<Id, N>
where
    Id: Eq + Hash + Copy,
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

impl<'a, Id: Copy + Eq + Hash, N: 'a> Iterator for NodeIterator<'a, Id, N> {
    type Item = Node<'a, Id, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, n)| Node::new(*id, n))
    }
}

pub trait EdgeContainer {
    type NId: Eq + Hash + Copy;
    type E;
    type EId: Eq + Hash + Copy;

    type EdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a>;

    fn len(&self) -> usize;

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn edge_data(&self, id: Self::EId) -> Option<&Self::E>;
    fn edge_data_mut(&mut self, id: Self::EId) -> Option<&mut Self::E>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId>;
    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;

    fn reverse_edge(&mut self, id: Self::EId) -> Option<()>;
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

use std::mem;

impl<NId, E> EdgeContainer for EdgeStableVec<NId, E>
where
    NId: Eq + Hash + Copy,
{
    type NId = NId;
    type E = E;
    type EId = usize;

    type EdgeIterator<'a> = EdgeIterator<'a, NId, E> where Self: 'a;

    fn edges(&self) -> EdgeIterator<NId, E> {
        EdgeIterator {
            inner: self.edges.iter().enumerate(),
        }
    }

    fn len(&self) -> usize {
        self.edges_len
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
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

    fn reverse_edge(&mut self, id: Self::EId) -> Option<()> {
        let edge = self.edges.get_mut(id)?.as_mut()?;
        mem::swap(&mut edge.u, &mut edge.v);

        Some(())
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

pub trait AdjContainer {
    type NId: Eq + Hash + Copy;
    type EId: Eq + Hash + Copy;

    type AdjIterator<'a>: Iterator<Item = (Self::EId, Self::NId)>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId>;
    fn degree(&self, u: Self::NId) -> usize;

    fn insert_node(&mut self, u: Self::NId);
    fn remove_node(&mut self, u: Self::NId);
    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::NId, Self::EId)>>;

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool;
    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
}

pub trait MultiAdjContainer: AdjContainer {
    type MultiEdgeIterator<'a>: Iterator<Item = (Self::NId, Self::EId)>
    where
        Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>>;
}

pub trait DirectedAdjContainer: AdjContainer {
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn out_degree(&self, u: Self::NId) -> usize;

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn in_degree(&self, u: Self::NId) -> usize;

    fn reverse_edge(&mut self, u: Self::NId, v: Self::NId, id: Self::EId);
}

pub trait UndirectedAdjContainer: AdjContainer {}
pub trait OrdinalAdjContainer: AdjContainer + Ordinal {}
pub trait KeyedAdjContainer: AdjContainer + Keyed {}

pub struct Di<AC: AdjContainer> {
    out_adj: AC,
    in_adj: AC,
}

impl<AC: AdjContainer> AdjContainer for Di<AC> {
    type NId = AC::NId;
    type EId = AC::EId;

    type AdjIterator<'a> = AC::AdjIterator<'a>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.out_adj.adj(u)
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        self.out_adj.between(u, v)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.out_adj.degree(u)
    }

    fn insert_node(&mut self, u: Self::NId) {
        self.out_adj.insert_node(u);
        self.in_adj.insert_node(u);
    }

    fn remove_node(&mut self, u: Self::NId) {
        self.out_adj.remove_node(u);
        self.in_adj.remove_node(u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::NId, Self::EId)>> {
        // TODO: how should this work at the Graph level? when graph iterates over adj() to
        // determine the edges to remove, it doesn't look at in edges, but here we clear both.
        // Should clear_node in adj container actually remove the adjacencies from neighboring
        // nodes as well (currently being done at Graph level) and return a vec of edge ids to
        // Graph can remove those from the edge container? I think this is the best option.
        let mut out_ids = self.out_adj.clear_node(u)?;
        let mut in_ids = self
            .in_adj
            .clear_node(u)
            .expect("out_adj and in_adj should both have the same nodes");
        out_ids.append(&mut in_ids);

        Some(out_ids)
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.out_adj.contains_edge(u, v)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.out_adj.insert_edge(u, v, edge_id);
        self.in_adj.insert_edge(v, u, edge_id);
    }

    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.out_adj.remove_edge(u, v, edge_id);
        self.in_adj.remove_edge(v, u, edge_id);
    }
}

impl<AC: AdjContainer> DirectedAdjContainer for Di<AC> {
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.out_adj.adj(u)
    }

    fn out_degree(&self, u: Self::NId) -> usize {
        self.out_adj.degree(u)
    }

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.in_adj.adj(u)
    }

    fn in_degree(&self, u: Self::NId) -> usize {
        self.in_adj.degree(u)
    }

    fn reverse_edge(&mut self, u: Self::NId, v: Self::NId, id: Self::EId) {
        self.out_adj.remove_edge(u, v, id);
        self.in_adj.remove_edge(v, u, id);

        self.out_adj.insert_edge(v, u, id);
        self.in_adj.insert_edge(u, v, id);
    }
}

impl<AC> MultiAdjContainer for Di<AC>
where
    AC: AdjContainer + MultiAdjContainer,
{
    type MultiEdgeIterator<'a> = AC::MultiEdgeIterator<'a> where Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>> {
        // TODO: what about in edges between u and v?
        // Should we just get rid of the multigraph type?
        self.out_adj.between_multi(u, v)
    }
}

// TODO: impl Ordinal when AC is Ordinal and same with Keyed
// Can we do a blanket impl here?

pub struct Un<AC: AdjContainer> {
    adj: AC,
}

impl<AC: AdjContainer> AdjContainer for Un<AC> {
    type NId = AC::NId;
    type EId = AC::EId;

    type AdjIterator<'a> = AC::AdjIterator<'a>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.adj.adj(u)
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        self.adj.between(u, v)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.adj.degree(u)
    }

    fn insert_node(&mut self, u: Self::NId) {
        self.adj.insert_node(u);
    }

    fn remove_node(&mut self, u: Self::NId) {
        self.adj.remove_node(u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::NId, Self::EId)>> {
        let ids = self.adj.clear_node(u)?;
        for &(v, edge_id) in &ids {
            self.remove_edge(v, u, edge_id);
        }
        Some(ids)
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.contains_edge(u, v)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.insert_edge(u, v, edge_id);
        self.adj.insert_edge(v, u, edge_id);
    }

    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.remove_edge(u, v, edge_id);
        self.adj.remove_edge(v, u, edge_id);
    }
}

impl<AC: AdjContainer> UndirectedAdjContainer for Un<AC> {}

pub struct AdjMap<NId, EId> {
    adj: HashMap<NId, HashMap<NId, EId>>,
}

impl<NId, EId> AdjContainer for AdjMap<NId, EId>
where
    NId: Eq + Hash + Copy,
    EId: Eq + Hash + Copy,
{
    type NId = NId;
    type EId = EId;

    type AdjIterator<'a> = AdjIterator<'a, NId, EId> where Self: 'a;

    fn adj(&self, u: Self::NId) -> Option<AdjIterator<NId, EId>> {
        Some(AdjIterator {
            inner: self.adj.get(&u)?.iter(),
        })
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        Some(self.adj.get(&u)?.get(&v)?.clone())
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.adj.get(&u).map_or(0, |adj_map| adj_map.len())
    }

    fn insert_node(&mut self, u: Self::NId) {
        self.adj.insert(u, HashMap::new());
    }

    fn remove_node(&mut self, u: Self::NId) {
        self.adj.remove(&u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::NId, Self::EId)>> {
        let u_adj = self.adj.get_mut(&u)?;
        let ids: Vec<_> = u_adj.iter().map(|(&v, &edge_id)| (v, edge_id)).collect();
        u_adj.clear();

        Some(ids)
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.get(&u).is_some() && self.adj[&u].contains_key(&v)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.get_mut(&u).unwrap().insert(v, edge_id);
    }

    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, _edge_id: Self::EId) {
        self.adj.get_mut(&u).unwrap().remove(&v);
    }
}

impl<NId, EId> Keyed for AdjMap<NId, EId> {}

impl<NId, EId> KeyedAdjContainer for AdjMap<NId, EId>
where
    NId: Eq + Hash + Copy,
    EId: Eq + Hash + Copy,
{
}

pub struct AdjIterator<'a, NId, EId> {
    inner: Iter<'a, NId, EId>,
}

impl<'a, NId, EId> Iterator for AdjIterator<'a, NId, EId>
where
    NId: 'a + Eq + Hash + Copy,
    EId: 'a + Eq + Hash + Copy,
{
    type Item = (EId, NId);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&v, &id)| (id, v))
    }
}

use crate::graph::structure::graph::{
    DirectedGraph, Graph, KeyedGraph, OrdinalGraph, UndirectedGraph,
};

pub struct CGraph<NC, EC, AC> {
    nodes: NC,
    edges: EC,
    adj: AC,
}

impl<NC, EC, AC> Graph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    type N = NC::N;
    type NId = NC::NId;
    type E = EC::E;
    type EId = EC::EId;

    type NodeIterator<'a> = NC::NodeIterator<'a> where Self: 'a;
    type EdgeIterator<'a> = EC::EdgeIterator<'a> where Self: 'a;
    type AdjIterator<'a> = GAdjIterator<'a, NC, EC, AC> where Self: 'a;

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
        let edge_ids: Vec<_> = self.adj.clear_node(u)?;
        for (_, edge_id) in edge_ids {
            self.edges
                .remove_edge(edge_id)
                .expect("Edge should be present");
        }
        Some(())
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.contains_edge(u, v)
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        self.edges.edge(id)
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        let edge_id = self.adj.between(u, v)?;
        self.edges.edge(edge_id)
    }

    fn edge_data(&self, id: Self::EId) -> Option<&Self::E> {
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
        Some(edge_id)
    }

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E> {
        let edge = self.edges.edge(id)?;
        let (u, v) = (edge.u(), edge.v());
        self.adj.remove_edge(u, v, id);
        self.edges.remove_edge(id)
    }

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a> {
        self.nodes.nodes()
    }

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a> {
        self.edges.edges()
    }

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        Some(GAdjIterator {
            graph: &self,
            inner: self.adj.adj(u)?,
        })
    }
}

impl<NC, EC, AC> DirectedGraph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: DirectedAdjContainer<NId = NC::NId, EId = EC::EId>,
{
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        Some(GAdjIterator {
            graph: &self,
            inner: self.adj.out_edges(u)?,
        })
    }

    fn out_degree(&self, u: Self::NId) -> usize {
        self.adj.out_degree(u)
    }

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        Some(GAdjIterator {
            graph: &self,
            inner: self.adj.in_edges(u)?,
        })
    }

    fn in_degree(&self, u: Self::NId) -> usize {
        self.adj.in_degree(u)
    }

    fn reverse_edge(&mut self, id: Self::EId) -> Option<()> {
        let edge = self.edge(id)?;
        self.adj.reverse_edge(edge.u(), edge.v(), id);

        self.edges.reverse_edge(id).unwrap();

        Some(())
    }
}

impl<NC, EC, AC> UndirectedGraph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: UndirectedAdjContainer<NId = NC::NId, EId = EC::EId>,
{
}

impl<NC, EC, AC> OrdinalGraph for CGraph<NC, EC, AC>
where
    NC: OrdinalNodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    fn insert_node(&mut self, node: Self::N) -> Self::NId {
        let id = self.nodes.insert_node(node);
        self.adj.insert_node(id);
        id
    }
}

impl<NC, EC, AC> KeyedGraph for CGraph<NC, EC, AC>
where
    NC: KeyedNodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: KeyedAdjContainer<NId = NC::NId, EId = EC::EId>,
{
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N> {
        let previous = self.remove_node(id);
        self.nodes.put_node(id, node);
        self.adj.insert_node(id);
        previous
    }
}

pub struct GAdjIterator<'a, NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    graph: &'a CGraph<NC, EC, AC>,
    inner: AC::AdjIterator<'a>,
}

impl<'a, NC, EC, AC> Iterator for GAdjIterator<'a, NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    type Item = (Edge<'a, NC::NId, EC::EId, EC::E>, Node<'a, NC::NId, NC::N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, v)| {
            (
                self.graph
                    .edges
                    .edge(id)
                    .expect("id from adj iterator must refer to real edge"),
                self.graph
                    .nodes
                    .node(v)
                    .expect("id from adj iterator must refer to real edge"),
            )
        })
    }
}

pub type DiGraph<NC, EC, AC> = CGraph<NC, EC, Di<AC>>;
pub type UnGraph<NC, EC, AC> = CGraph<NC, EC, Un<AC>>;

//  options for how to abstract directed vs undirected graphs:
//  1. UnGraph and DiGraph structs, where DiGraph stores in_edges and out_edges.
//      pros:
//          +
//      cons:
//          - sometimes you might not want to store separate in_edges even with a directed graph, if you don't care about the performance of reading in_edges
//          - have to write the same impl for most functions twice
//
// 2. Make Di<> and Un<> adjacency containers that hold other types of adj containers
//      pros:
//          + it's just better lol
//          + if you want to reduce memory / don't care about fast in_edges for a directed graph, just use raw adj container directly (AdjMap, AdjVec, AdjMatrix) without the Di<> or Un<>

// impl UnGraph where AC: Ordinal, NC: Ordinal
// impl UnGraph where AC: Keyed
/*
pub struct DiGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId=NC::NId>,
    AC: AdjContainer<NId=NC::NId, EId=EC::EId>,
{
    nodes: NC,
    edges: EC,
    out_adj: AC,
    in_adj: AC,
}

type BlahGraph = DiGraph<NodeMap<usize, ()>, EdgeStableVec<usize, ()>, AdjMap<usize, usize>>;*/
