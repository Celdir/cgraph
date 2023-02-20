use crate::graph::containers::adj::traits::{
    AdjContainer, DirectedAdjContainer, KeyedAdjContainer, MultiAdjContainer, OrdinalAdjContainer,
};
use crate::graph::errors::GraphError;
use crate::graph::traits::WithCapacity;
use std::default::Default;

pub struct Di<AC: AdjContainer> {
    out_adj: AC,
    in_adj: AC,
}

impl<AC: AdjContainer> AdjContainer for Di<AC> {
    type NId = AC::NId;
    type EId = AC::EId;

    type AdjIterator<'a> = AC::AdjIterator<'a>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.out_adj.adj(u)
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        self.out_adj.between(u, v)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.out_adj.degree(u)
    }

    fn register_node(&mut self, u: Self::NId) {
        self.out_adj.register_node(u);
        self.in_adj.register_node(u);
    }

    fn unregister_node(&mut self, u: Self::NId) {
        self.out_adj.unregister_node(u);
        self.in_adj.unregister_node(u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Result<Vec<(Self::EId, Self::NId)>, GraphError> {
        let mut out_ids = self.out_adj.clear_node(u)?;
        for &(edge_id, v) in &out_ids {
            self.in_adj.remove_adj(v, u, edge_id);
        }

        let mut in_ids = self
            .in_adj
            .clear_node(u)
            .expect("out_adj and in_adj should both have the same nodes");
        for &(edge_id, v) in &in_ids {
            self.out_adj.remove_adj(v, u, edge_id);
        }

        out_ids.append(&mut in_ids);

        Ok(out_ids)
    }

    fn contains_adj(&self, u: Self::NId, v: Self::NId) -> bool {
        self.out_adj.contains_adj(u, v)
    }

    fn insert_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.out_adj.insert_adj(u, v, edge_id);
        self.in_adj.insert_adj(v, u, edge_id);
    }

    fn remove_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.out_adj.remove_adj(u, v, edge_id);
        self.in_adj.remove_adj(v, u, edge_id);
    }
}

impl<AC: AdjContainer> DirectedAdjContainer for Di<AC> {
    fn out_adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.out_adj.adj(u)
    }

    fn out_degree(&self, u: Self::NId) -> usize {
        self.out_adj.degree(u)
    }

    fn in_adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.in_adj.adj(u)
    }

    fn in_degree(&self, u: Self::NId) -> usize {
        self.in_adj.degree(u)
    }

    fn reverse_adj(&mut self, u: Self::NId, v: Self::NId, id: Self::EId) {
        self.out_adj.remove_adj(u, v, id);
        self.in_adj.remove_adj(v, u, id);

        self.out_adj.insert_adj(v, u, id);
        self.in_adj.insert_adj(u, v, id);
    }
}

impl<AC> MultiAdjContainer for Di<AC>
where
    AC: MultiAdjContainer,
{
    type MultiEdgeIterator<'a> = AC::MultiEdgeIterator<'a> where Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>> {
        self.out_adj.between_multi(u, v)
    }
}

impl<AC> KeyedAdjContainer for Di<AC> where AC: KeyedAdjContainer {}
impl<AC> OrdinalAdjContainer for Di<AC> where AC: OrdinalAdjContainer {}

impl<AC> Default for Di<AC>
where
    AC: AdjContainer + Default,
{
    fn default() -> Self {
        Self {
            out_adj: AC::default(),
            in_adj: AC::default(),
        }
    }
}

impl<AC> WithCapacity for Di<AC>
where
    AC: AdjContainer + WithCapacity,
{
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            out_adj: AC::with_capacity(node_capacity, edge_capacity),
            in_adj: AC::with_capacity(node_capacity, edge_capacity),
        }
    }
}
