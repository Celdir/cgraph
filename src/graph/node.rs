use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone)]
pub struct Node<'a, NodeId, N> {
    id: NodeId,
    data: NodeData<'a, N>,
}

#[derive(Copy, Clone)]
pub enum NodeData<'a, N> {
    Ref(&'a N),
    Value(N),
}

impl<'a, NodeId: Copy, N> Node<'a, NodeId, N>
where
    Self: 'a,
{
    pub fn new(id: NodeId, data: NodeData<'a, N>) -> Node<'a, NodeId, N> {
        Node { id, data }
    }

    pub fn from_ref(id: NodeId, data: &'a N) -> Node<'a, NodeId, N> {
        Node {
            id,
            data: NodeData::Ref(data),
        }
    }

    pub fn from_value(id: NodeId, data: N) -> Node<'a, NodeId, N> {
        Node {
            id,
            data: NodeData::Value(data),
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn data(&self) -> &N {
        match &self.data {
            NodeData::Ref(r) => r,
            NodeData::Value(n) => &n,
        }
    }

    pub fn into_data(self) -> NodeData<'a, N> {
        self.data
    }
}

impl<'a, NodeId: Copy, N> Deref for Node<'a, NodeId, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.data()
    }
}

pub struct NodeMut<'a, NodeId, N> {
    id: NodeId,
    data: &'a mut N,
}

impl<'a, NodeId: Copy, N> NodeMut<'a, NodeId, N> {
    pub fn new(id: NodeId, data: &'a mut N) -> NodeMut<'a, NodeId, N> {
        NodeMut { id, data }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn data(&mut self) -> &mut N {
        self.data
    }

    pub fn into_data(self) -> &'a mut N {
        self.data
    }
}

impl<'a, NodeId, N> Deref for NodeMut<'a, NodeId, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, NodeId, N> DerefMut for NodeMut<'a, NodeId, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
