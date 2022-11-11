use crate::graph::containers::adj::traits::{
    AdjContainer, DirectedAdjContainer, KeyedAdjContainer, UndirectedAdjContainer,
};
use crate::graph::containers::edge::traits::EdgeContainer;
use crate::graph::containers::node::traits::{
    KeyedNodeContainer, NodeContainer, OrdinalNodeContainer,
};
use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::{DirectedGraph, Graph, KeyedGraph, OrdinalGraph, UndirectedGraph};
use std::default::Default;

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

impl<NC, EC, AC> Default for CGraph<NC, EC, AC>
where
    NC: Default,
    EC: Default,
    AC: Default,
{
    fn default() -> Self {
        Self {
            nodes: NC::default(),
            edges: EC::default(),
            adj: AC::default(),
        }
    }
}

impl<NC, EC, AC> CGraph<NC, EC, AC>
where
    NC: Default,
    EC: Default,
    AC: Default,
{
    pub fn new() -> Self {
        Self::default()
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
