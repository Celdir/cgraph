use std::ops::{Deref, DerefMut};

pub struct Node<'a, NodeId, N> {
    id: NodeId,
    data: &'a N,
}

impl<'a, NodeId: Copy, N> Node<'a, NodeId, N> {
    pub fn new(id: NodeId, data: &'a N) -> Node<'a, NodeId, N> {
        Node { id, data }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }

    pub fn data(&self) -> &'a N {
        self.data
    }
}

impl<'a, NodeId, N> Deref for Node<'a, NodeId, N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        self.data
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
