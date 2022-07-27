#[derive(PartialEq)]
pub enum Direction {
    In,
    Out,
    Undirected,
}

pub struct Edge<'a, NodeId, EdgeId, E> {
    id: EdgeId,
    origin: NodeId,
    destination: NodeId,
    data: &'a E,
}

impl<'a, NodeId: Copy, EdgeId: Copy, E> Edge<'a, NodeId, EdgeId, E> {
    pub fn new(id: EdgeId, origin: NodeId, destination: NodeId, data: &'a E) -> Edge<'a, NodeId, EdgeId, E> {
        Edge { id, origin, destination, data }
    }

    fn id(&self) -> EdgeId {
        self.id
    }

    fn origin(&self) -> NodeId {
        self.origin
    }
    
    fn u(&self) -> NodeId {
        self.origin
    }

    fn destination(&self) -> NodeId {
        self.destination
    }

    fn v(&self) -> NodeId {
        self.destination
    }

    fn data(&self) -> &'a E {
        &self.data
    }
}

pub type UniqueEdge<'a, NodeId, E> = Edge<'a, NodeId, (NodeId, NodeId), E>;
