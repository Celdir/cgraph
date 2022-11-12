use crate::graph::containers::node::traits::{NodeContainer, OrdinalNodeContainer};
use crate::graph::node::Node;
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

    fn nodes(&self) -> NodeIterator<N> {
        NodeIterator {
            inner: self.nodes.iter().enumerate(),
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
            Some(Some(node)) => Some(Node::new(id, node)),
            _ => None,
        }
    }

    fn node_data(&self, id: usize) -> Option<&N> {
        self.nodes.get(id)?.as_ref()
    }

    fn node_data_mut(&mut self, id: usize) -> Option<&mut N> {
        self.nodes.get_mut(id)?.as_mut()
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
                return Some(Node::new(id, node));
            }
        }
    }
}
