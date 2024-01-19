use crate::graph::node::{Node, NodeMut};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::iter::Iterator;

pub trait NodeContainer {
    type NId: Eq + Hash + Copy + Debug;
    type NodeHasher: Hasher + Default;
    type N;

    type NodeIterator<'a>: Iterator<Item = Node<'a, Self::NId, Self::N>>
    where
        Self: 'a;
    type NodeMutIterator<'a>: Iterator<Item = NodeMut<'a, Self::NId, Self::N>>
    where
        Self: 'a;

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a>;
    fn nodes_mut<'a>(&'a mut self) -> Self::NodeMutIterator<'a>;

    fn len(&self) -> usize;

    fn contains_node(&self, id: Self::NId) -> bool;
    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>>;
    fn node_mut(&mut self, id: Self::NId) -> Option<NodeMut<Self::NId, Self::N>>;

    fn remove_node(&mut self, id: Self::NId) -> Option<Self::N>;
}

pub trait OrdinalNodeContainer: NodeContainer {
    fn insert_node(&mut self, node: Self::N) -> Self::NId;
}

pub trait KeyedNodeContainer: NodeContainer {
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;
}
