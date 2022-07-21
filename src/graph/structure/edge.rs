#[derive(PartialEq)]
pub enum Direction {
    In,
    Out,
    Undirected,
}

pub struct Edge<NodeId, EdgeId, E> {
    id: EdgeId,
    origin: NodeId,
    destination: NodeId,
    data: E,
}

impl<NodeId: Copy, EdgeId: Copy, E> Edge<NodeId, EdgeId, E> {
    pub fn new(id: EdgeId, origin: NodeId, destination: NodeId, data: E) -> Edge<NodeId, EdgeId, E> {
        Edge { id, origin, destination, data }
    }

    pub fn id(&self) -> &EdgeId {
        &self.id
    }

    pub fn origin(&self) -> &NodeId {
        &self.origin
    }

    pub fn destination(&self) -> &NodeId {
        &self.destination
    }

    pub fn data(&self) -> &E {
        &self.data
    }

    pub fn into(self) -> E {
        self.data
    }
}

pub type UniqueEdge<NodeId, E> = Edge<NodeId, (NodeId, NodeId), E>;
