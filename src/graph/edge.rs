use std::ops::{Deref, DerefMut};

#[derive(PartialEq)]
pub enum Direction {
    In,
    Out,
    Undirected,
}

#[derive(Clone)]
pub struct Edge<'a, NodeId, EdgeId, E> {
    id: EdgeId,
    origin: NodeId,
    destination: NodeId,
    data: &'a E,
}

impl<'a, NodeId: Eq + Copy, EdgeId: Eq + Copy, E> Edge<'a, NodeId, EdgeId, E> {
    pub fn new(
        id: EdgeId,
        origin: NodeId,
        destination: NodeId,
        data: &'a E,
    ) -> Edge<'a, NodeId, EdgeId, E> {
        Edge {
            id,
            origin,
            destination,
            data,
        }
    }

    pub fn id(&self) -> EdgeId {
        self.id
    }

    pub fn origin(&self) -> NodeId {
        self.origin
    }

    pub fn u(&self) -> NodeId {
        self.origin
    }

    pub fn destination(&self) -> NodeId {
        self.destination
    }

    pub fn v(&self) -> NodeId {
        self.destination
    }

    pub fn other(&self, id: NodeId) -> NodeId {
        match id {
            x if x == self.origin => self.destination,
            x if x == self.destination => self.origin,
            _ => panic!("cannot give other of invalid node id"),
        }
    }

    pub fn data(&self) -> &'a E {
        self.data
    }
}

impl<'a, NodeId: Eq + Copy, EdgeId: Eq + Copy, E> Deref for Edge<'a, NodeId, EdgeId, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

pub struct EdgeMut<'a, NodeId, EdgeId, E> {
    id: EdgeId,
    origin: NodeId,
    destination: NodeId,
    data: &'a mut E,
}

impl<'a, NodeId: Eq + Copy, EdgeId: Eq + Copy, E> EdgeMut<'a, NodeId, EdgeId, E> {
    pub fn new(
        id: EdgeId,
        origin: NodeId,
        destination: NodeId,
        data: &'a mut E,
    ) -> EdgeMut<'a, NodeId, EdgeId, E> {
        EdgeMut {
            id,
            origin,
            destination,
            data,
        }
    }

    pub fn id(&self) -> EdgeId {
        self.id
    }

    pub fn origin(&self) -> NodeId {
        self.origin
    }

    pub fn u(&self) -> NodeId {
        self.origin
    }

    pub fn destination(&self) -> NodeId {
        self.destination
    }

    pub fn v(&self) -> NodeId {
        self.destination
    }

    pub fn other(&self, id: NodeId) -> NodeId {
        match id {
            x if x == self.origin => self.destination,
            x if x == self.destination => self.origin,
            _ => panic!("cannot give other of invalid node id"),
        }
    }

    pub fn data(&mut self) -> &mut E {
        self.data
    }

    pub fn into_data(self) -> &'a mut E {
        self.data
    }
}

impl<'a, NodeId: Eq + Copy, EdgeId: Eq + Copy, E> Deref for EdgeMut<'a, NodeId, EdgeId, E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

impl<'a, NodeId: Eq + Copy, EdgeId: Eq + Copy, E> DerefMut for EdgeMut<'a, NodeId, EdgeId, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}
