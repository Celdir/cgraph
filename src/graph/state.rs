use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use itertools::Itertools;

use super::{edge::Edge, node::Node, traits::Graph};

pub struct StateGraph<State, N, E, NV, EV, T, F>
where
    State: Eq + Hash + Copy + Debug,
    NV: Fn(State) -> N,
    EV: Fn(State, State) -> E,
    T: Fn(State) -> Vec<State>,
    F: Fn(State) -> bool,
{
    node_val: NV,
    edge_val: EV,
    transition: T,
    filter: F,
    _phantom: PhantomData<(State, N, E)>,
}

impl<State, N, E, NV, EV, T, F> StateGraph<State, N, E, NV, EV, T, F>
where
    State: Eq + Hash + Copy + Debug,
    NV: Fn(State) -> N,
    EV: Fn(State, State) -> E,
    T: Fn(State) -> Vec<State>,
    F: Fn(State) -> bool,
{
    pub fn new(node_val: NV, edge_val: EV, transition: T, filter: F) -> Self {
        Self {
            node_val,
            edge_val,
            transition,
            filter,
            _phantom: PhantomData::default(),
        }
    }
}

impl<State, N, E, NV, EV, T, F> Graph for StateGraph<State, N, E, NV, EV, T, F>
where
    State: Eq + Hash + Copy + Debug,
    NV: Fn(State) -> N,
    EV: Fn(State, State) -> E,
    T: Fn(State) -> Vec<State>,
    F: Fn(State) -> bool,
{
    type NId = State;
    type N = N;
    type EId = (State, State);
    type E = E;
    type AdjIterator<'a> = std::vec::IntoIter<
        (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    > where Self: 'a;
    type AdjIdsIterator<'a> = std::vec::IntoIter<(Self::EId, Self::NId)> where Self: 'a;

    fn contains_node(&self, id: Self::NId) -> bool {
        (self.filter)(id)
    }
    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>> {
        if !self.contains_node(id) {
            return None;
        }
        let val = (self.node_val)(id);
        Some(Node::from_value(id, val))
    }
    fn degree(&self, u: Self::NId) -> usize {
        (self.transition)(u).len()
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        if !self.contains_node(u) || !self.contains_node(v) {
            return false;
        }
        (self.transition)(u)
            .into_iter()
            .find(|&state| state == v)
            .is_some()
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        if !self.contains_edge(id.0, id.1) {
            return None;
        }
        let val = (self.edge_val)(id.0, id.1);
        Some(Edge::from_value(id, id.0, id.1, val))
    }
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        self.edge((u, v))
    }

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        if !self.contains_node(u) {
            return None;
        }
        Some(
            (self.transition)(u)
                .into_iter()
                .filter(|v| (self.filter)(*v))
                .filter_map(|v| Some((self.edge((u, v))?, self.node(v)?)))
                .collect_vec()
                .into_iter(),
        )
    }

    fn adj_ids<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIdsIterator<'a>> {
        if !self.contains_node(u) {
            return None;
        }
        Some(
            (self.transition)(u)
                .into_iter()
                .filter(|v| (self.filter)(*v))
                .map(|v| ((u, v), v))
                .collect_vec()
                .into_iter(),
        )
    }
}
