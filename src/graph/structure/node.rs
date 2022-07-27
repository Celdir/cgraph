pub struct Node<'a, NodeId, N> {
    id: NodeId,
    data: &'a N,
}

impl<'a, NodeId: Copy, N> Node<'a, NodeId, N> {
    pub fn new(id: NodeId, data: &'a N) -> Node<'a, NodeId, N> {
        Node{ id, data }
    }

    pub fn id(&self) -> NodeId {
        self.id
    }
    
    pub fn data(&self) -> &'a N {
        self.data
    }
}
