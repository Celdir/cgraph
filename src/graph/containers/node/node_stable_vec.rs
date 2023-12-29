use crate::graph::containers::node::traits::{NodeContainer, OrdinalNodeContainer};
use crate::graph::node::{Node, NodeMut};
use crate::graph::traits::WithCapacity;
use std::default::Default;
use std::iter;
use std::iter::Iterator;
use std::mem;
use std::slice;

#[derive(Default)]
pub struct NodeStableVec<N> {
    nodes: Vec<Option<N>>,
    nodes_len: usize,
}

impl<N> NodeContainer for NodeStableVec<N> {
    type NId = usize;
    type N = N;

    type NodeIterator<'a> = NodeIterator<'a, N> where Self: 'a;
    type NodeMutIterator<'a> = NodeMutIterator<'a, N> where Self: 'a;

    fn nodes(&self) -> NodeIterator<N> {
        NodeIterator {
            inner: self.nodes.iter().enumerate(),
        }
    }

    fn nodes_mut(&mut self) -> NodeMutIterator<N> {
        NodeMutIterator {
            inner: self.nodes.iter_mut().enumerate(),
        }
    }

    fn len(&self) -> usize {
        self.nodes_len
    }

    fn contains_node(&self, id: usize) -> bool {
        self.nodes.get(id).is_some() && self.nodes[id].is_some()
    }

    fn node(&self, id: usize) -> Option<Node<usize, N>> {
        match self.nodes.get(id) {
            Some(Some(node)) => Some(Node::from_ref(id, node)),
            _ => None,
        }
    }

    fn node_mut(&mut self, id: usize) -> Option<NodeMut<usize, N>> {
        match self.nodes.get_mut(id) {
            Some(Some(node)) => Some(NodeMut::new(id, node)),
            _ => None,
        }
    }

    fn remove_node(&mut self, id: usize) -> Option<N> {
        let node = mem::replace(self.nodes.get_mut(id)?, None)?;
        self.nodes_len -= 1;
        Some(node)
    }
}

impl<N> OrdinalNodeContainer for NodeStableVec<N> {
    fn insert_node(&mut self, node: N) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Some(node));
        self.nodes_len += 1;
        id
    }
}

impl<N> WithCapacity for NodeStableVec<N> {
    fn with_capacity(node_capacity: usize, _edge_capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(node_capacity),
            nodes_len: 0,
        }
    }
}

pub struct NodeIterator<'a, N> {
    inner: iter::Enumerate<slice::Iter<'a, Option<N>>>,
}

impl<'a, N> Iterator for NodeIterator<'a, N> {
    type Item = Node<'a, usize, N>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let node = opt.as_ref().unwrap();
                return Some(Node::from_ref(id, node));
            }
        }
    }
}

pub struct NodeMutIterator<'a, N> {
    inner: iter::Enumerate<slice::IterMut<'a, Option<N>>>,
}

impl<'a, N> Iterator for NodeMutIterator<'a, N> {
    type Item = NodeMut<'a, usize, N>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let node = opt.as_mut().unwrap();
                return Some(NodeMut::new(id, node));
            }
        }
    }
}
