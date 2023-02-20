use crate::graph::edge::{Edge, EdgeMut};
use crate::graph::errors::GraphError;
use std::hash::Hash;
use std::iter::Iterator;

pub trait EdgeContainer {
    type NId: Eq + Hash + Copy;
    type E;
    type EId: Eq + Hash + Copy;

    type EdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;
    type EdgeMutIterator<'a>: Iterator<Item = EdgeMut<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a>;
    fn edges_mut<'a>(&'a mut self) -> Self::EdgeMutIterator<'a>;

    fn len(&self) -> usize;

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Self::EId;
    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;

    fn reverse_edge(&mut self, id: Self::EId) -> Result<(), GraphError>;
}
