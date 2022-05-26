use crate::graph::structure::{Node, Edge};
use std::iter::Iterator;
use std::hash::Hash;

pub trait NodeContainer {
    type Id;
    type N;

    fn nodes(&self) -> Box<dyn Iterator<Item = Node<Self::Id, Self::N>> + '_>;

    fn nodes_len(&self) -> usize;

    fn get_node(&self, u: Self::Id) -> Option<Node<Self::Id, Self::N>>;

    fn insert_node(&mut self, node: Self::N) -> Self::Id;

    fn put_node(&mut self, id: Self::Id, node: Self::N) -> Option<N>; // returns old node data if present

    fn remove_node(&mut self, id: Self::Id) -> Self::N;
}

pub trait EdgeContainer {
    type NId;
    type EId;
    type E;

    fn edges(&self) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;

    fn adj(
        &self,
        u: Self::NId,
    ) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;

    fn edges_len(&self) -> usize;

    fn get_edge(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Self::EId;

    fn put_edge(&mut self, id: Self::EId, node: Self::E) -> Option<E>; // returns old edge data if present

    fn remove_edge(&mut self, id: Self::EId) -> Self::E;
}
