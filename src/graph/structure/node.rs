pub struct Node<'a, NodeId, N> {
    id: NodeId,
    data: &'a E,
}

impl<'a, NodeId, N> Node<'a, NodeId, N> {
    fn id(&self) -> NodeId {
        self.id
    }
    
    fn data(&self) -> &'a N {
        self.data
    }
}
