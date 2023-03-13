use crate::graph::containers::adj::traits::{AdjContainer, OrdinalAdjContainer, RawAdjContainer};
use crate::graph::errors::GraphError;
use crate::graph::traits::WithCapacity;
use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;
use std::slice::Iter;

#[derive(Default)]
pub struct FlatAdjList<EId> {
    start: Vec<usize>, // start[u] is start index of adj for outgoing edges of u
    // calcuate end[u] as start[u+1]
    adj: Vec<(usize, usize, EId)>,
}

impl<EId> FlatAdjList<EId>
where
    EId: Eq + Hash + Copy + Debug + Ord,
{
    fn edge_range(&self, u: usize) -> Option<(usize, usize)> {
        if u >= self.start.len() {
            return None;
        }
        Some((
            self.start[u],
            self.start.get(u + 1).copied().unwrap_or(self.adj.len()),
        ))
    }

    fn reindex_from(&mut self, u: usize, delta: i32) {
        if delta == 0 {
            return;
        }

        let delta_usize = delta.abs() as usize;
        for node in u..self.start.len() {
            match delta {
                0.. => {
                    self.start[node] += delta_usize;
                }
                _ => {
                    self.start[node] -= delta_usize;
                }
            }
        }
    }
}

impl<EId> AdjContainer for FlatAdjList<EId>
where
    EId: Eq + Hash + Copy + Debug + Ord,
{
    type NId = usize;
    type EId = EId;

    type AdjIterator<'a> = AdjIterator<'a, EId> where Self: 'a;

    fn adj(&self, u: usize) -> Option<AdjIterator<EId>> {
        let (s, e) = self.edge_range(u)?;
        Some(AdjIterator {
            inner: self.adj[s..e].iter(),
        })
    }

    // O(log(E))
    fn between(&self, u: usize, v: usize) -> Option<Self::EId> {
        match self
            .adj
            .binary_search_by(|probe| (probe.0, probe.1).cmp(&(u, v)))
        {
            Ok(pos) => Some(self.adj[pos].2),
            Err(_) => None,
        }
    }

    fn degree(&self, u: Self::NId) -> usize {
        if u >= self.start.len() {
            return 0;
        }
        let (s, e) = self.edge_range(u).unwrap_or((0, 0));
        e - s
    }

    fn register_node(&mut self, u: usize) {
        if u >= self.adj.len() {
            self.start.resize(u + 1, self.adj.len());
        }
    }

    fn unregister_node(&mut self, _u: Self::NId) {
        // do nothing, node has already been cleared and deleting the vec would messs up the
        // indices
    }

    fn clear_node(&mut self, u: Self::NId) -> Result<Vec<(Self::EId, Self::NId)>, GraphError> {
        let ids: Vec<_> = self
            .adj(u)
            .ok_or(GraphError::NodeNotFound(format!("{:?}", u)))?
            .collect();
        let (s, e) = self
            .edge_range(u)
            .ok_or(GraphError::NodeNotFound(format!("{:?}", u)))?;

        self.adj.drain(s..e);

        let start_delta = -((e - s) as i32);
        self.reindex_from(u + 1, start_delta);

        Ok(ids)
    }

    // O(log(E))
    fn contains_adj(&self, u: usize, v: usize) -> bool {
        match self
            .adj
            .binary_search_by(|probe| (probe.0, probe.1).cmp(&(u, v)))
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn insert_adj(&mut self, u: usize, v: usize, edge_id: Self::EId) {
        let item = (u, v, edge_id);
        match self.adj.binary_search(&item) {
            Ok(_) => {
                panic!("this edge id already exists in adjacency!");
            } // item already in adj
            Err(pos) => {
                self.adj.insert(pos, item);
                self.reindex_from(u + 1, 1);
            }
        }
    }

    // O(E)
    fn remove_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        let (s, e) = self.edge_range(u).unwrap();
        let index = s + self.adj[s..e]
            .iter()
            .position(|&tup| tup == (u, v, edge_id))
            .unwrap();
        self.adj.remove(index);
        self.reindex_from(u + 1, -1);
    }
}

impl<EId> OrdinalAdjContainer for FlatAdjList<EId> where EId: Eq + Hash + Copy + Debug + Ord {}
impl<EId> RawAdjContainer for FlatAdjList<EId> where EId: Eq + Hash + Copy + Debug + Ord {}

impl<EId> WithCapacity for FlatAdjList<EId> {
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            start: Vec::with_capacity(node_capacity),
            adj: Vec::with_capacity(edge_capacity),
        }
    }
}

pub struct AdjIterator<'a, EId> {
    inner: Iter<'a, (usize, usize, EId)>,
}

impl<'a, EId> Iterator for AdjIterator<'a, EId>
where
    EId: 'a + Eq + Hash + Copy + Debug,
{
    type Item = (EId, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|&(_, v, id)| (id, v))
    }
}
