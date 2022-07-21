use crate::graph::structure::node::Node;
use crate::graph::structure::edge::Edge;
use std::iter::Iterator;

pub trait NodeContainer {
    type Id;
    type N;

    fn nodes(&self) -> Box<dyn Iterator<Item = Node<Self::Id, Self::N>> + '_>;

    fn nodes_len(&self) -> usize;

    fn get_node(&self, u: Self::Id) -> Option<Node<Self::Id, Self::N>>;

    fn insert_node(&mut self, node: Self::N) -> Self::Id;

    fn put_node(&mut self, id: Self::Id, node: Self::N) -> Option<Self::N>; // returns old node data if present

    fn remove_node(&mut self, id: Self::Id) -> Option<Self::N>;
}

// We should store do all graph-related logic in the Graph struct, not in EdgeContainer.
// So we should simplify the functions in EdgeContainer.
// For example, insert_edge should have no knowledge of edge direction and should not try to link
// both ways.
// Come up with different functions so that Graph version of insert_edge decides how to link
// undirected vs directed edges
// store_edge (generate edge id and put somewhere) / link_edge (attach edge to specific nodes) ?
// or just insert_undirected_edge vs insert_directed_edge

pub trait EdgeContainer {
    type NId;
    type EId;
    type E;

    fn edges(&self) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;

    fn adj_edges(
        &self,
        u: Self::NId,
    ) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;

    fn edges_len(&self) -> usize;

    fn get_edge(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Self::EId;

    fn put_edge(&mut self, id: Self::EId, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::E>; // returns old edge data if present

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;
}
