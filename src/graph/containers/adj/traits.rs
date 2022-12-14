use std::hash::Hash;
use std::iter::Iterator;

pub trait AdjContainer {
    type NId: Eq + Hash + Copy;
    type EId: Eq + Hash + Copy;

    type AdjIterator<'a>: Iterator<Item = (Self::EId, Self::NId)>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId>;
    fn degree(&self, u: Self::NId) -> usize;

    fn insert_node(&mut self, u: Self::NId);
    fn remove_node(&mut self, u: Self::NId);
    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::EId, Self::NId)>>;

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool;
    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
}

pub trait MultiAdjContainer: AdjContainer {
    type MultiEdgeIterator<'a>: Iterator<Item = (Self::NId, Self::EId)>
    where
        Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>>;
}

pub trait DirectedAdjContainer: AdjContainer {
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn out_degree(&self, u: Self::NId) -> usize;

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn in_degree(&self, u: Self::NId) -> usize;

    fn reverse_edge(&mut self, u: Self::NId, v: Self::NId, id: Self::EId);
}

pub trait UndirectedAdjContainer: AdjContainer {}
pub trait OrdinalAdjContainer: AdjContainer {}
pub trait KeyedAdjContainer: AdjContainer {}
