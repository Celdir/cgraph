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
    pub fn new(id: EdgeId, origin: NodeId, destination: NodeId, data: &'a E) -> Edge<'a, NodeId, EdgeId, E> {
        Edge { id, origin, destination, data }
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
        &self.data
    }
}

pub type UniqueEdge<'a, NodeId, E> = Edge<'a, NodeId, (NodeId, NodeId), E>;
