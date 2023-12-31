use crate::graph::cgraph::CGraph;
use crate::graph::containers::adj::traits::RawAdjContainer;
use crate::graph::containers::edge::traits::EdgeContainer;
use crate::graph::containers::node::traits::NodeContainer;
use crate::graph::edge::{Edge, EdgeMut};
use crate::graph::errors::{FlowError, GraphError};
use crate::graph::node::Node;
use crate::graph::traits::{Graph, GraphIter, GraphMut};
use std::cmp::Ord;
use std::fmt::Debug;
use std::ops::{Add, Neg, Sub};

pub trait FlowGraph: GraphMut<E = Flow<<Self as FlowGraph>::FlowVal>> {
    type BackEdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;
    type BackAdjIterator<'a>: Iterator<
        Item = (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    >
    where
        Self: 'a;
    type ForwardEdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;
    type ForwardAdjIterator<'a>: Iterator<
        Item = (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    >
    where
        Self: 'a;

    type FlowVal: FlowValue;

    fn back_edges<'a>(&'a self) -> Self::BackEdgeIterator<'a>;
    fn forward_edges<'a>(&'a self) -> Self::ForwardEdgeIterator<'a>;

    fn back_adj<'a>(&'a self, u: Self::NId) -> Option<Self::BackAdjIterator<'a>>;
    fn forward_adj<'a>(&'a self, u: Self::NId) -> Option<Self::ForwardAdjIterator<'a>>;

    fn back_edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn back_edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>>;

    // adds flow edge and back edge, returns ids for both
    fn insert_flow_edge(
        &mut self,
        u: Self::NId,
        v: Self::NId,
        capacity: Self::FlowVal,
    ) -> Result<(Self::EId, Self::EId), GraphError>;

    // removes front and back edge, returns value for front edge and id + value for back edge
    fn remove_flow_edge(
        &mut self,
        id: Self::EId,
    ) -> Result<(Self::E, (Self::EId, Self::E)), GraphError>;

    fn increase_flow(&mut self, id: Self::EId, delta: Self::FlowVal) -> Result<(), GraphError>;

    fn reset_flow(&mut self);
}

pub trait FlowValue:
    Add<Output = Self> + Sub<Output = Self> + Neg<Output = Self> + Ord + Copy + Default + Debug
{
}
impl FlowValue for i8 {}
impl FlowValue for i16 {}
impl FlowValue for i32 {}
impl FlowValue for i64 {}
impl FlowValue for i128 {}
impl FlowValue for isize {}
//impl FlowValue for f32 {}
//impl FlowValue for f64 {}

#[derive(Default, Copy, Clone, Debug)]
pub struct Flow<V: FlowValue> {
    flow: V,
    capacity: V,
}

impl<V: FlowValue> Flow<V> {
    pub fn new(flow: V, capacity: V) -> Self {
        Self { flow, capacity }
    }

    pub fn new_forward(capacity: V) -> Self {
        Self {
            flow: V::default(),
            capacity,
        }
    }

    pub fn new_back() -> Self {
        Self {
            flow: V::default(),
            capacity: V::default(),
        }
    }

    pub fn flow(&self) -> V {
        self.flow
    }

    pub fn capacity(&self) -> V {
        self.capacity
    }

    pub fn residual(&self) -> V {
        self.capacity - self.flow
    }

    pub fn has_residual(&self) -> bool {
        self.residual() > V::default()
    }

    pub fn increase_flow(&mut self, delta: V) -> Result<(), String> {
        if self.flow + delta > self.capacity {
            return Err(format!("{:?}", &self));
        }
        self.flow = self.flow + delta;
        Ok(())
    }

    pub fn reset_flow(&mut self) {
        self.flow = V::default();
    }
}

impl<NC, EC, AC, T> FlowGraph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId, EId = usize, E = Flow<T>>,
    AC: RawAdjContainer<NId = NC::NId, EId = EC::EId>,
    T: FlowValue,
{
    type BackEdgeIterator<'a> = ModuloEdgeIterator<'a, Self> where Self: 'a;
    type BackAdjIterator<'a> = ModuloAdjIterator<'a, Self> where Self: 'a;
    type ForwardEdgeIterator<'a> = ModuloEdgeIterator<'a, Self> where Self: 'a;
    type ForwardAdjIterator<'a> = ModuloAdjIterator<'a, Self> where Self: 'a;
    type FlowVal = T;

    fn back_edges<'a>(&'a self) -> Self::BackEdgeIterator<'a> {
        ModuloEdgeIterator {
            inner: self.edges(),
            modulus: 2,
            remainder: 1,
        }
    }

    fn forward_edges<'a>(&'a self) -> Self::ForwardEdgeIterator<'a> {
        ModuloEdgeIterator {
            inner: self.edges(),
            modulus: 2,
            remainder: 0,
        }
    }

    fn back_adj<'a>(&'a self, u: Self::NId) -> Option<Self::BackAdjIterator<'a>> {
        Some(ModuloAdjIterator {
            inner: self.adj(u)?,
            modulus: 2,
            remainder: 1,
        })
    }
    fn forward_adj<'a>(&'a self, u: Self::NId) -> Option<Self::ForwardAdjIterator<'a>> {
        Some(ModuloAdjIterator {
            inner: self.adj(u)?,
            modulus: 2,
            remainder: 0,
        })
    }

    fn back_edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        // forward and back edges are adjacent, so flipping the parity of the id gives the back edge id
        self.edge(id ^ 1)
    }

    fn back_edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        // forward and back edges are adjacent, so flipping the parity of the id gives the back edge id
        self.edge_mut(id ^ 1)
    }

    // adds flow edge and back edge, returns ids for both
    fn insert_flow_edge(
        &mut self,
        u: Self::NId,
        v: Self::NId,
        capacity: Self::FlowVal,
    ) -> Result<(Self::EId, Self::EId), GraphError> {
        let forward_id = self.insert_edge(u, v, Flow::new_forward(capacity))?;
        let back_id = self
            .insert_edge(v, u, Flow::new_back())
            .expect("error inserting back edge in flow graph");
        Ok((forward_id, back_id))
    }

    // removes front and back edge, returns value for front edge and id + value for back edge
    fn remove_flow_edge(
        &mut self,
        id: Self::EId,
    ) -> Result<(Self::E, (Self::EId, Self::E)), GraphError> {
        let edge_val = self.remove_edge(id)?;
        let back_id = id ^ 1;
        Ok((edge_val, (back_id, self.remove_edge(back_id).unwrap())))
    }

    fn increase_flow(&mut self, id: Self::EId, delta: Self::FlowVal) -> Result<(), GraphError> {
        self.edge_mut(id)
            .ok_or(GraphError::EdgeNotFound(format!("{:?}", id)))?
            .increase_flow(delta)
            .map_err(|flow| FlowError::InsufficientCapacity(format!("{:?}", id), flow))?;
        self.back_edge_mut(id)
            .ok_or(FlowError::BackEdgeNotFound(format!("{:?}", id)))?
            .increase_flow(-delta)
            .expect("back edge must have residual capacity");

        Ok(())
    }

    fn reset_flow(&mut self) {
        for mut edge in self.edges_mut() {
            edge.reset_flow();
        }
    }
}

pub struct ModuloEdgeIterator<'a, G>
where
    G: 'a + GraphIter + GraphMut<EId = usize>,
{
    inner: G::EdgeIterator<'a>,
    modulus: usize,
    remainder: usize,
}

impl<'a, G> Iterator for ModuloEdgeIterator<'a, G>
where
    G: GraphIter + GraphMut<EId = usize>,
{
    type Item = Edge<'a, G::NId, G::EId, G::E>;

    fn next(&mut self) -> Option<Self::Item> {
        let edge = self.inner.next()?;
        if edge.id() % self.modulus == self.remainder {
            return Some(edge);
        }
        self.next()
    }
}

pub struct ModuloAdjIterator<'a, G>
where
    G: 'a + GraphMut<EId = usize>,
{
    inner: G::AdjIterator<'a>,
    modulus: usize,
    remainder: usize,
}

impl<'a, G> Iterator for ModuloAdjIterator<'a, G>
where
    G: GraphMut<EId = usize>,
{
    type Item = (Edge<'a, G::NId, G::EId, G::E>, Node<'a, G::NId, G::N>);

    fn next(&mut self) -> Option<Self::Item> {
        let adj = self.inner.next()?;
        if adj.0.id() % self.modulus == self.remainder {
            return Some(adj);
        }
        self.next()
    }
}
