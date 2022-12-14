use crate::graph::containers::adj::traits::{AdjContainer, KeyedAdjContainer};
use crate::graph::traits::WithCapacity;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;
use std::iter::Iterator;

#[derive(Default)]
pub struct AdjMap<NId, EId> {
    adj: HashMap<NId, HashMap<NId, EId>>,
}

impl<NId, EId> AdjContainer for AdjMap<NId, EId>
where
    NId: Eq + Hash + Copy,
    EId: Eq + Hash + Copy,
{
    type NId = NId;
    type EId = EId;

    type AdjIterator<'a> = AdjIterator<'a, NId, EId> where Self: 'a;

    fn adj(&self, u: Self::NId) -> Option<AdjIterator<NId, EId>> {
        Some(AdjIterator {
            inner: self.adj.get(&u)?.iter(),
        })
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        Some(self.adj.get(&u)?.get(&v)?.clone())
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.adj.get(&u).map_or(0, |adj_map| adj_map.len())
    }

    fn insert_node(&mut self, u: Self::NId) {
        self.adj.insert(u, HashMap::new());
    }

    fn remove_node(&mut self, u: Self::NId) {
        self.adj.remove(&u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::NId, Self::EId)>> {
        let u_adj = self.adj.get_mut(&u)?;
        let ids: Vec<_> = u_adj.iter().map(|(&v, &edge_id)| (v, edge_id)).collect();
        u_adj.clear();

        Some(ids)
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.get(&u).is_some() && self.adj[&u].contains_key(&v)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.get_mut(&u).unwrap().insert(v, edge_id);
    }

    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, _edge_id: Self::EId) {
        self.adj.get_mut(&u).unwrap().remove(&v);
    }
}

impl<NId, EId> KeyedAdjContainer for AdjMap<NId, EId>
where
    NId: Eq + Hash + Copy,
    EId: Eq + Hash + Copy,
{
}

impl<NId, EId> WithCapacity for AdjMap<NId, EId> {
    fn with_capacity(node_capacity: usize, _edge_capacity: usize) -> Self {
        Self {
            adj: HashMap::with_capacity(node_capacity),
        }
    }
}

pub struct AdjIterator<'a, NId, EId> {
    inner: Iter<'a, NId, EId>,
}

impl<'a, NId, EId> Iterator for AdjIterator<'a, NId, EId>
where
    NId: 'a + Eq + Hash + Copy,
    EId: 'a + Eq + Hash + Copy,
{
    type Item = (EId, NId);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&v, &id)| (id, v))
    }
}
