use crate::graph::traits::{KeyedGraph, OrdinalGraph, WithCapacity};

pub struct OrdinalGraphBuilder<G: OrdinalGraph + WithCapacity> {
    nodes: Vec<G::N>,
    edges: Vec<(G::NId, G::NId, G::E)>,
}

impl<G: OrdinalGraph + WithCapacity> OrdinalGraphBuilder<G> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn node(mut self, n: G::N) -> Self {
        self.nodes.push(n);
        self
    }

    pub fn nodes(mut self, mut nodes: Vec<G::N>) -> Self {
        self.nodes.append(&mut nodes);
        self
    }

    pub fn edge(mut self, u: G::NId, v: G::NId, e: G::E) -> Self {
        self.edges.push((u, v, e));
        self
    }

    pub fn edges(mut self, mut edges: Vec<(G::NId, G::NId, G::E)>) -> Self {
        self.edges.append(&mut edges);
        self
    }

    pub fn build(self) -> G {
        G::from_ordinal(self.nodes, self.edges)
    }
}

impl<G> OrdinalGraphBuilder<G>
where
    G: OrdinalGraph<N = ()> + WithCapacity,
{
    pub fn with_size(mut self, node_count: usize) -> Self {
        self.nodes.resize(node_count, ());
        self
    }
}

pub struct KeyedGraphBuilder<G: KeyedGraph + WithCapacity> {
    nodes: Vec<(G::NId, G::N)>,
    edges: Vec<(G::NId, G::NId, G::E)>,
}

impl<G: KeyedGraph + WithCapacity> KeyedGraphBuilder<G> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn node(mut self, key: G::NId, n: G::N) -> Self {
        self.nodes.push((key, n));
        self
    }

    pub fn nodes(mut self, mut nodes: Vec<(G::NId, G::N)>) -> Self {
        self.nodes.append(&mut nodes);
        self
    }

    pub fn edge(mut self, u: G::NId, v: G::NId, e: G::E) -> Self {
        self.edges.push((u, v, e));
        self
    }

    pub fn edges(mut self, mut edges: Vec<(G::NId, G::NId, G::E)>) -> Self {
        self.edges.append(&mut edges);
        self
    }

    pub fn build(self) -> G {
        G::from_keyed(self.nodes, self.edges)
    }
}
