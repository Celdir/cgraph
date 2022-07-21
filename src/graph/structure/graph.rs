/*use std::iter::Iterator;
use std::hash::Hash;

pub trait Graph<'a> {
    type Node: 'a;
    type Edge: 'a;
    type NodeId: Copy + Eq + Hash;
    type EdgeId: Copy + Eq + Hash;

    fn nodes(&self) -> Box<dyn Iterator<Item = (Self::NodeId, &Self::Node)> + '_>;
    fn edges(&self) -> Box<dyn Iterator<Item = (Self::NodeId, Self::NodeId, &Self::Edge)> + '_>;

    fn adj(
        &self,
        u: Self::NodeId,
    ) -> Box<dyn Iterator<Item = (Self::NodeId, Self::NodeId, &Self::Edge)> + '_>;

    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;

    fn get_node(&self, u: Self::NodeId) -> Option<&Self::Node>;
    fn get_edge(&self, u: Self::NodeId, v: Self::NodeId) -> Option<&Self::Edge>;

    fn insert_node(&mut self, node: Self::Node) -> Self::NodeId;
    fn insert_edge(&mut self, u: Self::NodeId, v: Self::NodeId, edge: Self::Edge);

    fn remove_node(&mut self, id: Self::NodeId) -> Self::Node;
    fn remove_edge(&mut self, id: Self::EdgeId) -> Self::Edge;
}

pub trait MapGraph<'a> {
    fn insert_node(&mut self, id: Self::NodeId, node: Self::Node) -> Self::NodeId;
}*/
/*
pub trait IterableGraph {
    type N;
    type NId = usize;
    type E;
    type EId;

    fn nodes(&self) -> Box<dyn Iterator<Item = Node<Self::NId, Self::N>> + '_>;

    fn nodes_len(&self) -> usize;

    fn get_node(&self, u: Self::NId) -> Option<Node<Self::NId, Self::N>>;

    fn edges(&self) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;

    fn edges_len(&self) -> usize;

    fn get_edge(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>>;

    fn adj_edges(
        &self,
        u: Self::NId,
    ) -> Box<dyn Iterator<Item = Edge<Self::NId, Self::EId, Self::E>> + '_>;
}

pub trait MutableGraph {
    fn insert_node(&mut self, node: Self::N) -> Self::Id;

    fn put_node(&mut self, id: Self::Id, node: Self::N) -> Option<Self::N>; // returns old node data if present

    fn remove_node(&mut self, id: Self::Id) -> Option<Self::N>;

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge: Self::E) -> Self::EId;

    fn put_edge(&mut self, id: Self::EId, u: Self::NId, v: Self::NId, edge: Self::E) -> Option<Self::E>; // returns old edge data if present

    fn remove_edge(&mut self, id: Self::EId) -> Option<Self::E>;
}

pub trait MapGraph {}
pub trait Directed {}
pub trait Undirected {}*/

use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::iter;
use crate::graph::structure::node::Node;
use crate::graph::structure::edge::Edge;
use std::ops::Index;

// TODO: implement pooling for re-using deleted node and edge ids
// alternative, just use hashmaps to store nodes and edges
//

struct EdgeIterator<'a, E> {
    u: usize,
    edges: &'a Vec<Option<Edge<usize, usize, E>>>,
    iter: Iter<'a, usize, usize>,
}

impl<'a, E> Iterator for EdgeIterator<'a, E> {
    type Item = &'a Edge<usize, usize, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(move |(&v, &id)| self.edges[id].as_ref().unwrap())
    }
}

pub struct VecGraph<N, E> {
    nodes: Vec<Option<N>>,
    edges: Vec<Option<Edge<usize, usize, E>>>,
    adj: Vec<HashMap<usize, usize>>,
}

impl<N, E> VecGraph<N, E> {
    fn new() -> VecGraph<N, E> {
        VecGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
            adj: Vec::new()
        }
    }

    fn get_node(&self, id: usize) -> Option<Node<usize, N>> {
        match self.nodes.get(id) {
            Some(Some(node)) => Some(Node::new(id, node)),
            _ => None
        }
    }

    fn insert_node(&mut self, node: N) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Some(node));
        id
    }

    fn remove_node(&mut self, id: usize) -> Option<N> {
        match self.nodes.get_mut(id) {
            Some(node) => std::mem::take(node),
            _ => None
        }
    }

    fn get_edge(&self, id: usize) -> Option<&Edge<usize, usize, E>> {
        self.edges.get(id)?.as_ref()
    }

    fn insert_edge(&mut self, u: usize, v: usize, edge: E) -> Option<usize> {
        if u >= self.nodes.len() || v >= self.nodes.len() {
            return None;
        }

        let id = self.edges.len();
        self.edges.push(Some(Edge::new(id, u, v, edge)));
        self.adj[u].insert(v, id);
        Some(id)
    }

    fn remove_edge(&mut self, id: usize) -> Option<E> {
        let edge = self.edges.remove(id)?;
        self.adj[*edge.origin()].remove(edge.destination());
        Some(edge.into())
    }

    fn between(&self, u: usize, v: usize) -> Option<&Edge<usize, usize, E>> {
        let &edge_id = self.adj.get(u)?.get(&v)?;
        self.edges.get(edge_id)?.as_ref()
    }

    fn adj_edges(&self, u: usize) -> Option<EdgeIterator<'_, E>> {
        Some(EdgeIterator {
            u,
            edges: &self.edges,
            iter: self.adj.get(u)?.iter()
        })
        //let edges = self.adj.get(u)?.iter();
        //Some(edges.map(move |(&v, &id)| Edge::new(id, u, v, &self.edges[id].as_ref().unwrap().e)))
    }
}

/*impl<N, E> Index<usize> for VecGraph<N, E> {
    type Output = EdgeIterator<'_, E>;

    fn index(&self, u: usize) -> &Self::Output{
        self.adj_edges(u).unwrap()
    }
}*/

impl<N, E> Index<(usize, usize)> for VecGraph<N, E> {
    type Output = Edge<usize, usize, E>;

    fn index(&self, (u, v): (usize, usize)) -> &Self::Output{
        self.between(u, v).unwrap()
    }
}
