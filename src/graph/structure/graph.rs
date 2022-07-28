/*use std::iter::Iterator;
use std::hash::Hash;

pub trait Graph<'a> {
    type Node: 'a;
    type Edge: 'a;
    type NodeId: Copy + Eq + Hash;
    type EdgeId: Copy + Eq + Hash;

    fn nodes(&self) -> Box<dyn Iterator<Item = (Self::NodeId, &Self::Node)> + '_>;
    fn edges(&self) -> Box<dyn Iterator<Item = (Self::NodeId, Self::NodeId, &Self::Edge)> + '_>;

    fn adj(
        &self,
        u: Self::NodeId,
    ) -> Box<dyn Iterator<Item = (Self::NodeId, Self::NodeId, &Self::Edge)> + '_>;

    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;

    fn get_node(&self, u: Self::NodeId) -> Option<&Self::Node>;
    fn get_edge(&self, u: Self::NodeId, v: Self::NodeId) -> Option<&Self::Edge>;

    fn insert_node(&mut self, node: Self::Node) -> Self::NodeId;
    fn insert_edge(&mut self, u: Self::NodeId, v: Self::NodeId, edge: Self::Edge);

    fn remove_node(&mut self, id: Self::NodeId) -> Self::Node;
    fn remove_edge(&mut self, id: Self::EdgeId) -> Self::Edge;
}

pub trait MapGraph<'a> {
    fn insert_node(&mut self, id: Self::NodeId, node: Self::Node) -> Self::NodeId;
}*/
/*
pub trait IterableGraph {
    type N;
    type NId = usize;
    type E;
    type EId;

    fn nodes(&self) -> Box<dyn Iterator<Item = Node<Self::NId, Self::N>> + '_>;

    fn nodes_len(&self) -> usize;

    fn get_node(&self, u: Self::NId) -> Option<Node<Self::NId, Self::N>>;

    fn edges(&self) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;

    fn edges_len(&self) -> usize;

    fn get_edge(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;

    fn adj_edges(
        &self,
        u: Self::NId,
    ) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;
}

pub trait MutableGraph {
    fn insert_node(&mut self, node: Self::N) -> Self::Id;

    fn put_node(&mut self, id: Self::Id, node: Self::N) -> Option<Self::N>; // returns old node data if present

    fn remove_node(&mut self, id: Self::Id) -> Option<Self::N>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Self::EId;

    fn put_edge(&mut self, id: Self::EId, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::E>; // returns old edge data if present

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;
}

pub trait MapGraph {}
pub trait Directed {}
pub trait Undirected {}*/
