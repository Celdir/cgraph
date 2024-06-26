use crate::graph::containers::adj::traits::{AdjContainer, KeyedAdjContainer, RawAdjContainer};
use crate::graph::errors::GraphError;
use crate::graph::traits::WithCapacity;

use ahash::AHashMap;
use std::collections::hash_map::Iter;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

#[derive(Default)]
pub struct AdjMap<NId, EId> {
    adj: AHashMap<NId, AHashMap<NId, EId>>,
}

impl<NId, EId> AdjContainer for AdjMap<NId, EId>
where
    NId: Eq + Hash + Copy + Debug,
    EId: Eq + Hash + Copy + Debug,
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

    fn register_node(&mut self, u: Self::NId) {
        self.adj.insert(u, AHashMap::new());
    }

    fn unregister_node(&mut self, u: Self::NId) {
        self.adj.remove(&u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Result<Vec<(Self::EId, Self::NId)>, GraphError> {
        let u_adj = self
            .adj
            .get_mut(&u)
            .ok_or_else(|| GraphError::NodeNotFound(format!("{:?}", u)))?;
        let ids: Vec<_> = u_adj.iter().map(|(&v, &edge_id)| (edge_id, v)).collect();
        u_adj.clear();

        Ok(ids)
    }

    fn contains_adj(&self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.get(&u).is_some() && self.adj[&u].contains_key(&v)
    }

    fn insert_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.adj.get_mut(&u).unwrap().insert(v, edge_id);
    }

    fn remove_adj(&mut self, u: Self::NId, v: Self::NId, _edge_id: Self::EId) {
        self.adj.get_mut(&u).unwrap().remove(&v);
    }
}

impl<NId, EId> KeyedAdjContainer for AdjMap<NId, EId>
where
    NId: Eq + Hash + Copy + Debug,
    EId: Eq + Hash + Copy + Debug,
{
}

impl<NId, EId> RawAdjContainer for AdjMap<NId, EId>
where
    NId: Eq + Hash + Copy + Debug,
    EId: Eq + Hash + Copy + Debug,
{
}

impl<NId, EId> WithCapacity for AdjMap<NId, EId> {
    fn with_capacity(node_capacity: usize, _edge_capacity: usize) -> Self {
        Self {
            adj: AHashMap::with_capacity(node_capacity),
        }
    }
}

pub struct AdjIterator<'a, NId, EId> {
    inner: Iter<'a, NId, EId>,
}

impl<'a, NId, EId> Iterator for AdjIterator<'a, NId, EId>
where
    NId: 'a + Eq + Hash + Copy + Debug,
    EId: 'a + Eq + Hash + Copy + Debug,
{
    type Item = (EId, NId);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(&v, &id)| (id, v))
    }
}
