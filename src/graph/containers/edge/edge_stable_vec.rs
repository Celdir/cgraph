use crate::graph::containers::edge::traits::EdgeContainer;
use crate::graph::edge::Edge;
use std::hash::Hash;
use std::iter;
use std::iter::Iterator;
use std::mem;
use std::slice;
use std::default::Default;

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
    NId: Eq + Hash + Copy,
{
    type NId = NId;
    type E = E;
    type EId = usize;

    type EdgeIterator<'a> = EdgeIterator<'a, NId, E> where Self: 'a;

    fn edges(&self) -> EdgeIterator<NId, E> {
        EdgeIterator {
            inner: self.edges.iter().enumerate(),
        }
    }

    fn len(&self) -> usize {
        self.edges_len
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        match self.edges.get(id) {
            Some(Some(edge)) => Some(Edge::new(id, edge.u, edge.v, &edge.e)),
            _ => None,
        }
    }

    fn edge_data(&self, id: usize) -> Option<&E> {
        Some(&self.edges.get(id)?.as_ref()?.e)
    }

    fn edge_data_mut(&mut self, id: usize) -> Option<&mut E> {
        Some(&mut self.edges.get_mut(id)?.as_mut()?.e)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::EId> {
        let id = self.edges.len();
        self.edges.push(Some(InternalEdge { u, v, e: edge }));
        self.edges_len += 1;
        Some(id)
    }

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E> {
        let internal_edge = mem::replace(self.edges.get_mut(id)?, None)?;
        self.edges_len -= 1;
        Some(internal_edge.e)
    }

    fn reverse_edge(&mut self, id: Self::EId) -> Option<()> {
        let edge = self.edges.get_mut(id)?.as_mut()?;
        mem::swap(&mut edge.u, &mut edge.v);

        Some(())
    }
}

pub struct EdgeIterator<'a, NId, E> {
    inner: iter::Enumerate<slice::Iter<'a, Option<InternalEdge<NId, E>>>>,
}

impl<'a, NId, E> Iterator for EdgeIterator<'a, NId, E>
where
    NId: 'a + Eq + Hash + Copy,
    E: 'a,
{
    type Item = Edge<'a, NId, usize, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let edge = opt.as_ref().unwrap();
                return Some(Edge::new(id, edge.u, edge.v, &edge.e));
            }
        }
    }
}
