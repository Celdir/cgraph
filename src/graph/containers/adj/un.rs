use crate::graph::containers::adj::traits::{
    AdjContainer, KeyedAdjContainer, MultiAdjContainer, OrdinalAdjContainer, UndirectedAdjContainer,
};
use crate::graph::traits::WithCapacity;
use std::default::Default;

pub struct Un<AC: AdjContainer> {
    adj: AC,
}

impl<AC: AdjContainer> AdjContainer for Un<AC> {
    type NId = AC::NId;
    type EId = AC::EId;

    type AdjIterator<'a> = AC::AdjIterator<'a>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.adj.adj(u)
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        self.adj.between(u, v)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.adj.degree(u)
    }

    fn insert_node(&mut self, u: Self::NId) {
        self.adj.insert_node(u);
    }

    fn remove_node(&mut self, u: Self::NId) {
        self.adj.remove_node(u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::EId, Self::NId)>> {
        let ids = self.adj.clear_node(u)?;
        for &(edge_id, v) in &ids {
            self.adj.remove_edge(v, u, edge_id);
        }
        Some(ids)
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.contains_edge(u, v)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.insert_edge(u, v, edge_id);
        self.adj.insert_edge(v, u, edge_id);
    }

    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.remove_edge(u, v, edge_id);
        self.adj.remove_edge(v, u, edge_id);
    }
}

impl<AC: AdjContainer> UndirectedAdjContainer for Un<AC> {}

impl<AC> MultiAdjContainer for Un<AC>
where
    AC: MultiAdjContainer,
{
    type MultiEdgeIterator<'a> = AC::MultiEdgeIterator<'a> where Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>> {
        self.adj.between_multi(u, v)
    }
}

impl<AC> KeyedAdjContainer for Un<AC> where AC: KeyedAdjContainer {}
impl<AC> OrdinalAdjContainer for Un<AC> where AC: OrdinalAdjContainer {}

impl<AC> Default for Un<AC>
where
    AC: AdjContainer + Default,
{
    fn default() -> Self {
        Self { adj: AC::default() }
    }
}

impl<AC> WithCapacity for Un<AC>
where
    AC: AdjContainer + WithCapacity,
{
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            adj: AC::with_capacity(node_capacity, edge_capacity),
        }
    }
}
