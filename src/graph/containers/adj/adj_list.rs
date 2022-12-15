use crate::graph::containers::adj::traits::{AdjContainer, OrdinalAdjContainer};
use crate::graph::traits::WithCapacity;
use std::default::Default;
use std::hash::Hash;
use std::iter::Iterator;
use std::slice::Iter;

#[derive(Default)]
pub struct AdjList<EId> {
    adj: Vec<Vec<(EId, usize)>>,
}

impl<EId> AdjContainer for AdjList<EId>
where
    EId: Eq + Hash + Copy,
{
    type NId = usize;
    type EId = EId;

    type AdjIterator<'a> = AdjIterator<'a, EId> where Self: 'a;

    fn adj(&self, u: usize) -> Option<AdjIterator<EId>> {
        Some(AdjIterator {
            inner: self.adj.get(u)?.iter(),
        })
    }

    // O(deg(u))
    fn between(&self, u: usize, v: usize) -> Option<Self::EId> {
        Some(self.adj.get(u)?.iter().find(|&&(_, nid)| nid == v)?.0)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.adj.get(u).map_or(0, |adj_list| adj_list.len())
    }

    fn register_node(&mut self, u: usize) {
        if u >= self.adj.len() {
            self.adj.resize(u + 1, Vec::new())
        }
    }

    fn unregister_node(&mut self, _u: Self::NId) {
        // do nothing, node has already been cleared and deleting the vec would messs up the
        // indices
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::EId, Self::NId)>> {
        let u_adj = self.adj.get_mut(u)?;
        let ids: Vec<_> = u_adj.iter().copied().collect();
        u_adj.clear();

        Some(ids)
    }

    // O(deg(u))
    fn contains_adj(&self, u: usize, v: usize) -> bool {
        self.adj.get(u).is_some() && self.adj[u].iter().find(|&&(_, nid)| nid == v).is_some()
    }

    fn insert_adj(&mut self, u: usize, v: usize, edge_id: Self::EId) {
        self.adj.get_mut(u).unwrap().push((edge_id, v));
    }

    // O(deg(u))
    fn remove_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        let index = self.adj[u]
            .iter()
            .position(|&pair| pair == (edge_id, v))
            .unwrap();
        self.adj.get_mut(u).unwrap().swap_remove(index);
    }
}

impl<EId> OrdinalAdjContainer for AdjList<EId> where EId: Eq + Hash + Copy {}

impl<EId> WithCapacity for AdjList<EId> {
    fn with_capacity(node_capacity: usize, _edge_capacity: usize) -> Self {
        Self {
            adj: Vec::with_capacity(node_capacity),
        }
    }
}

pub struct AdjIterator<'a, EId> {
    inner: Iter<'a, (EId, usize)>,
}

impl<'a, EId> Iterator for AdjIterator<'a, EId>
where
    EId: 'a + Eq + Hash + Copy,
{
    type Item = (EId, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }
}
