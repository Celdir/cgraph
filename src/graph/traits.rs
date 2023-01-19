use crate::graph::edge::Edge;
use crate::graph::node::{Node, NodeMut};
use std::hash::Hash;

pub trait Graph {
    type N;
    type NId: Eq + Hash + Copy;
    type E;
    type EId: Eq + Hash + Copy;

    type NodeIterator<'a>: Iterator<Item = Node<'a, Self::NId, Self::N>>
    where
        Self: 'a;
    type NodeMutIterator<'a>: Iterator<Item = NodeMut<'a, Self::NId, Self::N>>
    where
        Self: 'a;
    type EdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;
    type AdjIterator<'a>: Iterator<
        Item = (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    >
    where
        Self: 'a;

    fn len(&self) -> (usize, usize);

    fn contains_node(&self, id: Self::NId) -> bool;
    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>>;
    fn node_data(&self, id: Self::NId) -> Option<&Self::N>;
    fn node_data_mut(&mut self, id: Self::NId) -> Option<&mut Self::N>;
    fn degree(&self, u: Self::NId) -> usize;
    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N>;
    fn clear_node(&mut self, id: Self::NId) -> Option<()>;

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool;
    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn edge_data(&self, id: Self::EId) -> Option<&Self::E>;
    fn edge_data_mut(&mut self, id: Self::EId) -> Option<&mut Self::E>;
    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId>;
    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a>;
    fn nodes_mut<'a>(&'a mut self) -> Self::NodeMutIterator<'a>;
    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a>;
    // Returns out edges for directed graph or all edges for undirected
    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
}

pub trait OrdinalGraph: Graph {
    fn insert_node(&mut self, node: Self::N) -> Self::NId;

    fn from_ordinal(nodes: Vec<Self::N>, edges: Vec<(Self::NId, Self::NId, Self::E)>) -> Self
    where
        Self: WithCapacity;
}

pub trait KeyedGraph: Graph {
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;

    fn from_keyed(nodes: Vec<(Self::NId, Self::N)>, edges: Vec<(Self::NId, Self::NId, Self::E)>) -> Self
    where
        Self: WithCapacity;
}

pub trait DirectedGraph: Graph {
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn out_degree(&self, u: Self::NId) -> usize;

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn in_degree(&self, u: Self::NId) -> usize;

    fn reverse_edge(&mut self, id: Self::EId) -> Option<()>;
}

pub trait UndirectedGraph: Graph {}

pub trait MultiGraph: Graph {
    type MultiEdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>>;
}

pub trait WithCapacity {
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self;
}
