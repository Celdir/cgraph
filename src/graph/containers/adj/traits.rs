use crate::graph::errors::GraphError;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

pub trait AdjContainer {
    type NId: Eq + Hash + Copy + Debug;
    type EId: Eq + Hash + Copy + Debug;

    type AdjIterator<'a>: Iterator<Item = (Self::EId, Self::NId)>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId>;
    fn degree(&self, u: Self::NId) -> usize;

    fn register_node(&mut self, u: Self::NId);
    fn unregister_node(&mut self, u: Self::NId);
    fn clear_node(&mut self, u: Self::NId) -> Result<Vec<(Self::EId, Self::NId)>, GraphError>;

    fn contains_adj(&self, u: Self::NId, v: Self::NId) -> bool;
    fn insert_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
    fn remove_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId);
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
    fn out_adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn out_degree(&self, u: Self::NId) -> usize;

    fn in_adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn in_degree(&self, u: Self::NId) -> usize;

    fn reverse_adj(&mut self, u: Self::NId, v: Self::NId, id: Self::EId);
}

pub trait UndirectedAdjContainer: AdjContainer {}
pub trait OrdinalAdjContainer: AdjContainer {}
pub trait KeyedAdjContainer: AdjContainer {}

// Raw Adj Containers are base-level containers like adj list. These are directed but don't provide
// in_adj computation because that requires extra memory.
pub trait RawAdjContainer: AdjContainer {}
