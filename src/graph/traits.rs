use crate::graph::builder::{KeyedGraphBuilder, OrdinalGraphBuilder};
use crate::graph::edge::{Edge, EdgeMut};
use crate::graph::errors::GraphError;
use crate::graph::node::{Node, NodeMut};
use std::fmt::Debug;
use std::hash::Hash;

pub trait Graph {
    type N;
    type NId: Eq + Hash + Copy + Debug;
    type E;
    type EId: Eq + Hash + Copy + Debug;

    type NodeIterator<'a>: Iterator<Item = Node<'a, Self::NId, Self::N>>
    where
        Self: 'a;

    type EdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;

    type AdjIterator<'a>: Iterator<
        Item = (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    >
    where
        Self: 'a;
    type AdjIdsIterator<'a>: Iterator<Item = (Self::EId, Self::NId)>
    where
        Self: 'a;

    fn len(&self) -> (usize, usize);

    fn contains_node(&self, id: Self::NId) -> bool;
    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>>;
    fn degree(&self, u: Self::NId) -> usize;

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool;
    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a>;

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a>;

    // Returns out edges for directed graph or all edges for undirected
    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn adj_ids<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIdsIterator<'a>>;
}

pub trait GraphMut: Graph {
    type NodeMutIterator<'a>: Iterator<Item = NodeMut<'a, Self::NId, Self::N>>
    where
        Self: 'a;
    type EdgeMutIterator<'a>: Iterator<Item = EdgeMut<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;
    type AdjMutIterator<'a>: Iterator<
        Item = (
            EdgeMut<'a, Self::NId, Self::EId, Self::E>,
            NodeMut<'a, Self::NId, Self::N>,
        ),
    >
    where
        Self: 'a;

    fn node_mut(&mut self, id: Self::NId) -> Option<NodeMut<Self::NId, Self::N>>;
    fn edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>>;
    fn between_mut(
        &mut self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>>;


    fn insert_edge(
        &mut self,
        u: Self::NId,
        v: Self::NId,
        edge: Self::E,
    ) -> Result<Self::EId, GraphError>;
    fn remove_edge(&mut self, id: Self::EId) -> Result<Self::E, GraphError>;

    fn remove_node(&mut self, id: Self::NId) -> Result<Self::N, GraphError>;
    fn clear_node(&mut self, id: Self::NId) -> Result<(), GraphError>;


    fn nodes_mut<'a>(&'a mut self) -> Self::NodeMutIterator<'a>;
    fn edges_mut<'a>(&'a mut self) -> Self::EdgeMutIterator<'a>;
    fn adj_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>>;
}

pub trait OrdinalGraph: GraphMut {
    fn insert_node(&mut self, node: Self::N) -> Self::NId;

    fn from_ordinal(nodes: Vec<Self::N>, edges: Vec<(Self::NId, Self::NId, Self::E)>) -> Self
    where
        Self: WithCapacity;

    fn builder() -> OrdinalGraphBuilder<Self>
    where
        Self: WithCapacity + Sized,
    {
        OrdinalGraphBuilder::<Self>::new()
    }
}

pub trait KeyedGraph: GraphMut {
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N>;

    fn from_keyed(
        nodes: Vec<(Self::NId, Self::N)>,
        edges: Vec<(Self::NId, Self::NId, Self::E)>,
    ) -> Self
    where
        Self: WithCapacity;

    fn builder() -> KeyedGraphBuilder<Self>
    where
        Self: WithCapacity + Sized,
    {
        KeyedGraphBuilder::<Self>::new()
    }
}

pub trait DirectedGraph: Graph {
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn out_degree(&self, u: Self::NId) -> usize;

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>>;
    fn in_degree(&self, u: Self::NId) -> usize;
}

pub trait DirectedGraphMut: GraphMut + DirectedGraph {
    fn out_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>>;
    fn in_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>>;

    fn reverse_edge(&mut self, id: Self::EId) -> Result<(), GraphError>;
}

pub trait UndirectedGraph: Graph {}
pub trait RawGraph: Graph {}

pub trait MultiGraph: Graph {
    type MultiEdgeIterator<'a>: Iterator<Item = Edge<'a, Self::NId, Self::EId, Self::E>>
    where
        Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>>;
}

pub trait WithCapacity {
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self;
}
