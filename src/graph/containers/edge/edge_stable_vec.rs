use crate::graph::containers::edge::traits::EdgeContainer;
use crate::graph::edge::{Edge, EdgeMut};
use crate::graph::errors::GraphError;
use crate::graph::traits::WithCapacity;
use std::default::Default;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
use std::iter::Iterator;
use std::mem;
use std::slice;

#[derive(Default)]
pub struct EdgeStableVec<NId, E> {
    edges: Vec<Option<InternalEdge<NId, E>>>,
    edges_len: usize,
}

struct InternalEdge<Id, E> {
    u: Id,
    v: Id,
    e: E,
}

impl<NId, E> EdgeContainer for EdgeStableVec<NId, E>
where
    NId: Eq + Hash + Copy + Debug,
{
    type NId = NId;
    type E = E;
    type EId = usize;

    type EdgeIterator<'a> = EdgeIterator<'a, NId, E> where Self: 'a;
    type EdgeMutIterator<'a> = EdgeMutIterator<'a, NId, E> where Self: 'a;

    fn edges(&self) -> EdgeIterator<NId, E> {
        EdgeIterator {
            inner: self.edges.iter().enumerate(),
        }
    }

    fn edges_mut(&mut self) -> EdgeMutIterator<NId, E> {
        EdgeMutIterator {
            inner: self.edges.iter_mut().enumerate(),
        }
    }

    fn len(&self) -> usize {
        self.edges_len
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        match self.edges.get(id) {
            Some(Some(edge)) => Some(Edge::from_ref(id, edge.u, edge.v, &edge.e)),
            _ => None,
        }
    }

    fn edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        match self.edges.get_mut(id) {
            Some(Some(edge)) => Some(EdgeMut::new(id, edge.u, edge.v, &mut edge.e)),
            _ => None,
        }
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Self::EId {
        let id = self.edges.len();
        self.edges.push(Some(InternalEdge { u, v, e: edge }));
        self.edges_len += 1;
        id
    }

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E> {
        let internal_edge = mem::replace(self.edges.get_mut(id)?, None)?;
        self.edges_len -= 1;
        Some(internal_edge.e)
    }

    fn reverse_edge(&mut self, id: Self::EId) -> Result<(), GraphError> {
        let edge = self
            .edges
            .get_mut(id)
            .ok_or(GraphError::EdgeNotFound(format!("{:?}", id)))?
            .as_mut()
            .ok_or(GraphError::EdgeNotFound(format!("{:?}", id)))?;
        mem::swap(&mut edge.u, &mut edge.v);

        Ok(())
    }
}

impl<NId, E> WithCapacity for EdgeStableVec<NId, E> {
    fn with_capacity(_node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            edges: Vec::with_capacity(edge_capacity),
            edges_len: 0,
        }
    }
}

pub struct EdgeIterator<'a, NId, E> {
    inner: iter::Enumerate<slice::Iter<'a, Option<InternalEdge<NId, E>>>>,
}

impl<'a, NId, E> Iterator for EdgeIterator<'a, NId, E>
where
    NId: 'a + Eq + Hash + Copy + Debug,
    E: 'a,
{
    type Item = Edge<'a, NId, usize, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let edge = opt.as_ref().unwrap();
                return Some(Edge::from_ref(id, edge.u, edge.v, &edge.e));
            }
        }
    }
}

pub struct EdgeMutIterator<'a, NId, E> {
    inner: iter::Enumerate<slice::IterMut<'a, Option<InternalEdge<NId, E>>>>,
}

impl<'a, NId, E> Iterator for EdgeMutIterator<'a, NId, E>
where
    NId: 'a + Eq + Hash + Copy + Debug,
    E: 'a,
{
    type Item = EdgeMut<'a, NId, usize, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let edge = opt.as_mut().unwrap();
                return Some(EdgeMut::new(id, edge.u, edge.v, &mut edge.e));
            }
        }
    }
}
