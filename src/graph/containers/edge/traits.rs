use crate::graph::edge::Edge;
use std::hash::Hash;
use std::iter::Iterator;

pub trait EdgeContainer {
    type NId: Eq + Hash + Copy;
    type E;
    type EId: Eq + Hash + Copy;

    type EdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a>;

    fn len(&self) -> usize;

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn edge_data(&self, id: Self::EId) -> Option<&Self::E>;
    fn edge_data_mut(&mut self, id: Self::EId) -> Option<&mut Self::E>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId>;
    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;

    fn reverse_edge(&mut self, id: Self::EId) -> Option<()>;
}
