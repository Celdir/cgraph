use crate::graph::structure::edge::Edge;
use crate::graph::structure::node::Node;

pub trait Graph<'a> {
    type N: 'a;
    type NId;
    type E: 'a;
    type EId;

    type NodeIterator: Iterator<Item = Node<'a, Self::NId, Self::N>>;
    type EdgeIterator: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>;
    type AdjIterator: Iterator<
        Item = (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    >;

    fn len(&self) -> (usize, usize);

    fn contains_node(&self, id: Self::NId) -> bool;
    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>>;
    fn node_data(&self, id: Self::NId) -> Option<&Self::N>;
    fn node_data_mut(&mut self, id: Self::NId) -> Option<&mut Self::N>;
    fn degree(&self, u: Self::NId) -> usize;
    fn insert_node(&mut self, node: Self::N) -> Self::NId;
    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N>;
    fn clear_node(&mut self, id: Self::NId) -> Option<()>;

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool;
    fn edge(&self, id: Self::EId) -> Option<Edge<Self::EId, Self::EId, Self::E>>;
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn edge_data(&self, id: Self::EId) -> Option<&Self::E>;
    fn edge_data_mut(&mut self, id: Self::EId) -> Option<&mut Self::E>;
    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId>;
    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;

    fn nodes(&self) -> Self::NodeIterator;
    fn edges(&self) -> Self::EdgeIterator;
    // Returns out edges for directed graph or all edges for undirected
    fn adj(&self, u: Self::NId) -> Option<Self::AdjIterator>;
}

pub trait DirectedGraph<'a> {
    type N: 'a;
    type NId;
    type E: 'a;
    type EId;

    type AdjIterator: Iterator<
        Item = (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    >;

    fn out_edges(&self, u: Self::NId) -> Option<Self::AdjIterator>;
    fn out_degree(&self, u: Self::NId) -> usize;

    fn in_edges(&self, u: Self::NId) -> Option<Self::AdjIterator>;
    fn in_degree(&self, u: Self::NId) -> usize;
}

pub trait MultiGraph<'a> {
    type N: 'a;
    type NId;
    type E: 'a;
    type EId;

    type EdgeIterator: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>;

    fn between_multi(&self, u: Self::NId, v: Self::NId) -> Option<Self::EdgeIterator>;
}

pub trait KeyedGraph<'a> {
    type N: 'a;
    type NId;

    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;
}
