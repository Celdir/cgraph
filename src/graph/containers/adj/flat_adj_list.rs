use crate::graph::containers::adj::traits::{AdjContainer, OrdinalAdjContainer, RawAdjContainer};
use crate::graph::errors::GraphError;
use crate::graph::traits::WithCapacity;
use std::cell::{Cell, RefCell};
use std::cmp::Ord;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;
use std::slice::Iter;

#[derive(Default)]
pub struct FlatAdjList<EId> {
    start: RefCell<Vec<usize>>, // start[u] is start index of adj for outgoing edges of u
    // calcuate end[u] as start[u+1]
    adj: RefCell<Vec<(usize, usize, EId)>>,
    needs_reindex: Cell<bool>,
}

impl<EId> FlatAdjList<EId>
where
    EId: Eq + Hash + Copy + Debug + Ord,
{
    fn edge_range(&self, u: usize) -> Option<(usize, usize)> {
        let start = self.start.borrow();
        if u >= start.len() {
            return None;
        }
        Some((
            start[u],
            start.get(u + 1).copied().unwrap_or(self.adj.borrow().len()),
        ))
    }

    fn reindex(&self) {
        let mut start = self.start.borrow_mut();
        let mut adj = self.adj.borrow_mut();
        adj.sort();
        adj.dedup();

        start.fill(adj.len());
        for i in (0..adj.len()).rev() {
            start[adj[i].0] = i;
        }
        for i in (0..start.len()).rev() {
            if start[i] == adj.len() && i + 1 < start.len() {
                start[i] = start[i + 1];
            }
        }
        self.needs_reindex.set(false);
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
        if self.needs_reindex.get() {
            self.reindex();
        }
        let (s, e) = self.edge_range(u)?;
        let adj = unsafe { self.adj.try_borrow_unguarded().unwrap() };
        Some(AdjIterator {
            inner: adj[s..e].iter(),
        })
    }

    // O(log(E))
    fn between(&self, u: usize, v: usize) -> Option<Self::EId> {
        if self.needs_reindex.get() {
            self.reindex();
        }
        match self
            .adj
            .borrow()
            .binary_search_by(|probe| (probe.0, probe.1).cmp(&(u, v)))
        {
            Ok(pos) => Some(self.adj.borrow()[pos].2),
            Err(_) => None,
        }
    }

    fn degree(&self, u: Self::NId) -> usize {
        if self.needs_reindex.get() {
            self.reindex();
        }
        println!("{:?}", self.adj.borrow());
        println!("{:?}", self.start.borrow());
        if u >= self.start.borrow().len() {
            return 0;
        }
        let (s, e) = self.edge_range(u).unwrap_or((0, 0));
        e - s
    }

    fn register_node(&mut self, u: usize) {
        let len = self.adj.borrow().len();
        if u >= len {
            self.start.get_mut().resize(u + 1, len);
        }
    }

    fn unregister_node(&mut self, _u: Self::NId) {
        // do nothing, node has already been cleared and deleting the vec would messs up the
        // indices
    }

    fn clear_node(&mut self, u: Self::NId) -> Result<Vec<(Self::EId, Self::NId)>, GraphError> {
        if u >= self.start.borrow().len() {
            return Err(GraphError::NodeNotFound(format!("{:?}", u)));
        }
        let ids: Vec<_> = self
            .adj
            .borrow()
            .iter()
            .filter(|(node, _, _)| u == *node)
            .map(|&(_, v, id)| (id, v))
            .collect();
        let adj: Vec<_> = self
            .adj
            .take()
            .into_iter()
            .filter(|&(node, _, _)| u != node)
            .collect();
        self.adj.replace(adj);
        self.needs_reindex.set(true);

        Ok(ids)
    }

    // O(log(E))
    fn contains_adj(&self, u: usize, v: usize) -> bool {
        if self.needs_reindex.get() {
            self.reindex();
        }
        match self
            .adj
            .borrow()
            .binary_search_by(|probe| (probe.0, probe.1).cmp(&(u, v)))
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn insert_adj(&mut self, u: usize, v: usize, edge_id: Self::EId) {
        let item = (u, v, edge_id);
        self.adj.get_mut().push(item);
        self.needs_reindex.set(true);
    }

    // O(E)
    fn remove_adj(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        let idx = self
            .adj
            .borrow()
            .iter()
            .position(|&e| e == (u, v, edge_id))
            .unwrap();
        self.adj.get_mut().remove(idx);
        self.needs_reindex.set(true);
    }
}

impl<EId> OrdinalAdjContainer for FlatAdjList<EId> where EId: Eq + Hash + Copy + Debug + Ord {}
impl<EId> RawAdjContainer for FlatAdjList<EId> where EId: Eq + Hash + Copy + Debug + Ord {}

impl<EId> WithCapacity for FlatAdjList<EId> {
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            start: Vec::with_capacity(node_capacity).into(),
            adj: Vec::with_capacity(edge_capacity).into(),
            needs_reindex: false.into(),
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
