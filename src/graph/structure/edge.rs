pub enum Direction {
    Directed,
    Undirected,
}

pub struct Edge<'a, NodeId, EdgeId, E> {
    id: EdgeId,
    origin: NodeId,
    destination: NodeId,
    data: &'a E,
    direction: Direction,
}

impl<'a, NodeId, EdgeId, E> Edge<'a, NodeId, EdgeId, E> {
    fn id(&self) -> EdgeId {
        self.id
    }

    fn origin(&self) -> NodeId {
        self.origin
    }

    fn destination(&self) -> NodeId {
        self.destination
    }

    fn data(&self) -> &'a E {
        self.data
    }

    fn direction(&self) -> Direction {
        self.direction
    }

    fn is_directed(&self) -> bool {
        self.direction == Direction::Directed
    }

    fn is_undirected(&self) -> bool {
        self.direction == Direction::Undirected
    }
}

pub type Edge<'a, NodeId, E> = Edge<'a, NodeId, (NodeId, NodeId), E>;
