use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use crate::graph::traits::{KeyedGraph, OrdinalGraph, WithCapacity};

use crate::graph::keyed::Keyed;

use super::cgraph::CGraph;
use super::containers::adj::adj_list::AdjList;
use super::containers::adj::di::Di;
use super::containers::adj::flat_adj_list::FlatAdjList;
use super::containers::adj::traits::{AdjContainer, RawAdjContainer};
use super::containers::adj::un::Un;
use super::containers::edge::edge_stable_vec::EdgeStableVec;
use super::containers::edge::traits::EdgeContainer;
use super::containers::node::node_stable_vec::NodeStableVec;
use super::containers::node::traits::{NodeContainer, OrdinalNodeContainer};
use super::flow::{Flow, FlowGraph, FlowValue};

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

pub struct FlowGraphBuilder<G: FlowGraph + OrdinalGraph + WithCapacity> {
    nodes: Vec<G::N>,
    edges: Vec<(G::NId, G::NId, G::FlowVal)>,
}

impl<G: FlowGraph + OrdinalGraph + WithCapacity> FlowGraphBuilder<G> {
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

    pub fn edge(mut self, u: G::NId, v: G::NId, cap: G::FlowVal) -> Self {
        self.edges.push((u, v, cap));
        self
    }

    pub fn edges(mut self, mut edges: Vec<(G::NId, G::NId, G::FlowVal)>) -> Self {
        self.edges.append(&mut edges);
        self
    }

    pub fn build(self) -> G {
        let mut graph = G::with_capacity(self.nodes.len(), self.edges.len());
        for n in self.nodes {
            graph.insert_node(n);
        }
        for (u, v, cap) in self.edges {
            graph
                .insert_flow_edge(u, v, cap)
                .expect("node ids should refer to valid nodes");
        }
        graph
    }
}

impl<G> FlowGraphBuilder<G>
where
    G: FlowGraph + OrdinalGraph<N = ()> + WithCapacity,
{
    pub fn with_size(mut self, node_count: usize) -> Self {
        self.nodes.resize(node_count, ());
        self
    }
}

pub struct GraphBuilder<N, E> {
    _n: PhantomData<N>,
    _e: PhantomData<E>,
}

pub struct ShapeChoice<NC, EC, AC>
where
    NC: NodeContainer + WithCapacity,
    EC: EdgeContainer<NId = NC::NId> + WithCapacity,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId> + WithCapacity,
{
    _nc: PhantomData<NC>,
    _ec: PhantomData<EC>,
    _ac: PhantomData<AC>,
}

pub struct DirectionChoice<NC, EC, AC>
where
    NC: NodeContainer + WithCapacity,
    EC: EdgeContainer<NId = NC::NId> + WithCapacity,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId> + WithCapacity,
{
    _nc: PhantomData<NC>,
    _ec: PhantomData<EC>,
    _ac: PhantomData<AC>,
}

pub struct KeyChoice<NC, EC, AC>
where
    NC: NodeContainer + WithCapacity,
    EC: EdgeContainer<NId = NC::NId> + WithCapacity,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId> + WithCapacity,
{
    _nc: PhantomData<NC>,
    _ec: PhantomData<EC>,
    _ac: PhantomData<AC>,
}

impl<N, E> GraphBuilder<N, E> {
    pub fn new() -> ShapeChoice<NodeStableVec<N>, EdgeStableVec<usize, E>, AdjList<usize>> {
        ShapeChoice {
            _nc: PhantomData::default(),
            _ec: PhantomData::default(),
            _ac: PhantomData::default(),
        }
    }
}

impl<NC, EC, AC> ShapeChoice<NC, EC, AC>
where
    NC: NodeContainer<NId = usize> + WithCapacity,
    EC: EdgeContainer<NId = NC::NId> + WithCapacity,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId> + WithCapacity,
{
    pub fn adj_flat(self) -> DirectionChoice<NC, EC, FlatAdjList<EC::EId>>
    where
        EC::EId: Ord,
    {
        DirectionChoice {
            _nc: PhantomData::default(),
            _ec: PhantomData::default(),
            _ac: PhantomData::default(),
        }
    }

    pub fn adj_list(self) -> DirectionChoice<NC, EC, AdjList<EC::EId>> {
        DirectionChoice {
            _nc: PhantomData::default(),
            _ec: PhantomData::default(),
            _ac: PhantomData::default(),
        }
    }
}

impl<NC, EC, AC> DirectionChoice<NC, EC, AC>
where
    NC: NodeContainer<NId = usize> + WithCapacity,
    EC: EdgeContainer<NId = NC::NId, EId = usize> + WithCapacity,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId> + WithCapacity,
{
    pub fn directed(self) -> KeyChoice<NC, EC, Di<AC>> {
        KeyChoice {
            _nc: PhantomData::default(),
            _ec: PhantomData::default(),
            _ac: PhantomData::default(),
        }
    }
    pub fn di(self) -> KeyChoice<NC, EC, Di<AC>> {
        self.directed()
    }

    pub fn undirected(self) -> KeyChoice<NC, EC, Un<AC>> {
        KeyChoice {
            _nc: PhantomData::default(),
            _ec: PhantomData::default(),
            _ac: PhantomData::default(),
        }
    }
    pub fn un(self) -> KeyChoice<NC, EC, Un<AC>> {
        self.undirected()
    }

    pub fn flow(self) -> FlowGraphBuilder<CGraph<NC, EdgeStableVec<NC::NId, Flow<EC::E>>, AC>>
    where
        EC::E: FlowValue,
        NC: OrdinalNodeContainer,
        AC: RawAdjContainer,
    {
        FlowGraphBuilder::new()
    }
}

impl<NC, EC, AC> KeyChoice<NC, EC, AC>
where
    NC: NodeContainer + WithCapacity,
    EC: EdgeContainer<NId = NC::NId> + WithCapacity,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId> + WithCapacity,
{
    pub fn keyed<T: Eq + Hash + Copy + Debug>(
        self,
    ) -> KeyedGraphBuilder<Keyed<CGraph<NC, EC, AC>, T>>
    where
        NC: OrdinalNodeContainer,
    {
        KeyedGraphBuilder::<Keyed<CGraph<NC, EC, AC>, T>>::new()
    }

    pub fn ordinal(self) -> OrdinalGraphBuilder<CGraph<NC, EC, AC>>
    where
        NC: OrdinalNodeContainer,
    {
        OrdinalGraphBuilder::<CGraph<NC, EC, AC>>::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{graph::builder::GraphBuilder, iter::dfs::dfs, algo::flow::dinic::dinic};

    #[test]
    fn build_flat_directed_ordinal() {
        let graph = GraphBuilder::<(), ()>::new()
            .adj_flat()
            .di()
            .ordinal()
            .with_size(5)
            .edge(0, 1, ())
            .edge(2, 1, ())
            .edge(3, 2, ())
            .edge(4, 0, ())
            .build();

        let path_from_4: Vec<_> = dfs(&graph, 4).map(|(_, node)| node.id()).collect();
        assert_eq!(path_from_4, vec![4, 0, 1]);

        let path_from_3: Vec<_> = dfs(&graph, 3).map(|(_, node)| node.id()).collect();
        assert_eq!(path_from_3, vec![3, 2, 1]);
    }

    #[test]
    fn build_flow() {
        let mut graph = GraphBuilder::<(), isize>::new()
            .adj_flat()
            .flow()
            .with_size(4)
            .edge(0, 1, 1)
            .edge(0, 2, 1)
            .edge(1, 2, 1)
            .edge(1, 3, 1)
            .edge(2, 3, 1)
            .build();
        assert_eq!(dinic(&mut graph, 0, 3).unwrap(), 2);
    }
}
