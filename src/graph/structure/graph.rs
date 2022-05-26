use std::iter::Iterator;
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
}
