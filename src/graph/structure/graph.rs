use crate::graph::structure::edge::Edge;
use crate::graph::structure::node::Node;
use std::hash::Hash;

pub trait Graph<'a> {
    type N: 'a;
    type NId: Eq + Hash + Copy;
    type E: 'a;
    type EId: Eq + Hash + Copy;

    type NodeIterator: Iterator<Item = Node<'a, Self::NId, Self::N>>;
    type EdgeIterator: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>;
    type AdjIterator: Iterator<
        Item = (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    >;

    fn len(&'a self) -> (usize, usize);

    fn contains_node(&'a self, id: Self::NId) -> bool;
    fn node(&'a self, id: Self::NId) -> Option<Node<Self::NId, Self::N>>;
    fn node_data(&'a self, id: Self::NId) -> Option<&Self::N>;
    fn node_data_mut(&mut self, id: Self::NId) -> Option<&mut Self::N>;
    fn degree(&'a self, u: Self::NId) -> usize;
    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N>;
    fn clear_node(&mut self, id: Self::NId) -> Option<()>;

    fn contains_edge(&'a self, u: Self::NId, v: Self::NId) -> bool;
    fn edge(&'a self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn between(&'a self, u: Self::NId, v: Self::NId)
        -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn edge_data(&'a self, id: Self::EId) -> Option<&Self::E>;
    fn edge_data_mut(&mut self, id: Self::EId) -> Option<&mut Self::E>;
    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId>;
    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;

    fn nodes(&'a self) -> Self::NodeIterator;
    fn edges(&'a self) -> Self::EdgeIterator;
    // Returns out edges for directed graph or all edges for undirected
    fn adj(&'a self, u: Self::NId) -> Option<Self::AdjIterator>;
}

pub trait OrdinalGraph<'a>: Graph<'a> {
    fn insert_node(&mut self, node: Self::N) -> Self::NId;
}

pub trait KeyedGraph<'a>: Graph<'a> {
    fn put_node(&'a mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;
}

pub trait DirectedGraph<'a>: Graph<'a> {
    fn out_edges(&'a self, u: Self::NId) -> Option<Self::AdjIterator>;
    fn out_degree(&'a self, u: Self::NId) -> usize;

    fn in_edges(&'a self, u: Self::NId) -> Option<Self::AdjIterator>;
    fn in_degree(&'a self, u: Self::NId) -> usize;

    fn reverse(&'a self) -> Self;
    fn reverse_edge(&'a mut self, id: Self::EId) -> Option<()>;
}

pub trait UndirectedGraph<'a>: Graph<'a> {}

pub trait MultiGraph<'a>: Graph<'a> {
    type MultiEdgeIterator: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>;

    fn between_multi(&'a self, u: Self::NId, v: Self::NId) -> Option<Self::MultiEdgeIterator>;
}
