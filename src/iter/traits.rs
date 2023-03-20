use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;

pub trait Traversal<'a, G>: Iterator<Item = Self::StepItem>
where
    G: 'a + Graph,
{
    type StepItem;

    fn is_visited(&self, node_id: G::NId) -> bool;
    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>>;

    fn current_node(&self) -> Option<Node<'a, G::NId, G::N>>;

    fn path_to(&mut self, target: G::NId) -> Option<Path<'a, G>>;
}

pub struct Path<'a, G>
where
    G: 'a + Graph,
{
    path: Vec<(
        Option<Edge<'a, G::NId, G::EId, G::E>>,
        Node<'a, G::NId, G::N>,
    )>,
}

impl<'a, G> Path<'a, G>
where
    G: 'a + Graph,
{
    pub fn new(
        path: Vec<(
            Option<Edge<'a, G::NId, G::EId, G::E>>,
            Node<'a, G::NId, G::N>,
        )>,
    ) -> Self {
        Self { path }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &Node<'a, G::NId, G::N>> {
        self.path.iter().map(|(_, node)| node)
    }

    pub fn edges(&self) -> impl Iterator<Item = &Edge<'a, G::NId, G::EId, G::E>> {
        self.path
            .iter()
            .map(|(edge_opt, _)| edge_opt)
            .filter(|edge_opt| edge_opt.is_some())
            .map(|edge_opt| edge_opt.as_ref().unwrap())
    }
}

impl<'a, G> IntoIterator for Path<'a, G>
where
    G: 'a + Graph,
{
    type Item = (
        Option<Edge<'a, G::NId, G::EId, G::E>>,
        Node<'a, G::NId, G::N>,
    );
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}
