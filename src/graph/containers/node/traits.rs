use crate::graph::node::Node;
use std::hash::Hash;
use std::iter::Iterator;

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

pub trait OrdinalNodeContainer: NodeContainer {
    fn insert_node(&mut self, node: Self::N) -> Self::NId;
}

pub trait KeyedNodeContainer: NodeContainer {
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;
}
