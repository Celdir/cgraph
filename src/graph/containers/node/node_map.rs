use crate::graph::containers::node::traits::{KeyedNodeContainer, NodeContainer};
use crate::graph::node::Node;
use crate::graph::traits::WithCapacity;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::default::Default;
use std::hash::Hash;
use std::iter::Iterator;

#[derive(Default)]
pub struct NodeMap<Id, N> {
    nodes: HashMap<Id, N>,
}

impl<Id, N> NodeContainer for NodeMap<Id, N>
where
    Id: Eq + Hash + Copy,
{
    type NId = Id;
    type N = N;

    type NodeIterator<'a> = NodeIterator<'a, Id, N> where Self: 'a;

    fn nodes(&self) -> NodeIterator<Id, N> {
        NodeIterator {
            inner: self.nodes.iter(),
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn contains_node(&self, id: Id) -> bool {
        self.nodes.contains_key(&id)
    }

    fn node(&self, id: Id) -> Option<Node<Id, N>> {
        self.nodes.get(&id).map(|n| Node::new(id, n))
    }

    fn node_data(&self, id: Id) -> Option<&N> {
        self.nodes.get(&id)
    }

    fn node_data_mut(&mut self, id: Id) -> Option<&mut N> {
        self.nodes.get_mut(&id)
    }

    fn remove_node(&mut self, id: Id) -> Option<N> {
        self.nodes.remove(&id)
    }
}

impl<Id, N> KeyedNodeContainer for NodeMap<Id, N>
where
    Id: Eq + Hash + Copy,
{
    fn put_node(&mut self, id: Id, node: N) -> Option<N> {
        let previous = self.remove_node(id);
        self.nodes.insert(id, node);

        previous
    }
}

impl<Id, N> WithCapacity for NodeMap<Id, N> {
    fn with_capacity(node_capacity: usize, _edge_capacity: usize) -> Self {
        Self {
            nodes: HashMap::with_capacity(node_capacity),
        }
    }
}

pub struct NodeIterator<'a, Id, N> {
    inner: Iter<'a, Id, N>,
}

impl<'a, Id: Copy + Eq + Hash, N: 'a> Iterator for NodeIterator<'a, Id, N> {
    type Item = Node<'a, Id, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, n)| Node::new(*id, n))
    }
}
