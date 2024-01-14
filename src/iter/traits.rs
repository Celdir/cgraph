use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;

use ahash::AHashMap;

pub trait Tree<'a, G>
where
    G: 'a + Graph,
{
    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>>;
    fn path_to(&self, target: G::NId) -> Option<Path<'a, G>>;
}

pub trait Traversal<'a, G>: Tree<'a, G> + Iterator<Item = Self::StepItem>
where
    G: 'a + Graph,
{
    type StepItem;

    fn is_visited(&self, node_id: G::NId) -> bool;
    fn current_node(&self) -> Option<Node<'a, G::NId, G::N>>;

    // iterates until target is found and then returns path
    fn find_path_to(&mut self, target: G::NId) -> Option<Path<'a, G>>;
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

pub struct PathTree<'a, G>
where
    G: 'a + Graph,
{
    graph: &'a G,
    parent: AHashMap<G::NId, Option<G::EId>>,
}

impl<'a, G> Tree<'a, G> for PathTree<'a, G>
where
    G: 'a + Graph,
{
    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        let &edge_id = self.parent.get(&id)?.as_ref()?;
        self.graph.edge(edge_id)
    }

    fn path_to(&self, target: G::NId) -> Option<Path<'a, G>> {
        if !self.parent.contains_key(&target) {
            return None;
        }

        let mut path = Vec::new();
        let mut node_id_opt = Some(target);
        while node_id_opt.is_some() {
            let node_id = node_id_opt.unwrap();
            let node = self.graph.node(node_id).expect("node should exist");
            let edge = self.parent_edge(node_id);
            node_id_opt = edge.as_ref().map(|e| e.other(node_id));

            path.push((edge, node));
        }
        path.reverse();

        Some(Path::new(path))
    }
}

impl<'a, G> PathTree<'a, G>
where
    G: 'a + Graph,
{
    pub fn new(graph: &'a G) -> PathTree<'a, G> {
        PathTree {
            graph,
            parent: AHashMap::new(),
        }
    }

    pub fn contains_node(&self, id: G::NId) -> bool {
        self.parent.contains_key(&id)
    }

    pub fn insert_parent(&mut self, id: G::NId, parent: Option<G::EId>) {
        self.parent.insert(id, parent);
    }
}

pub struct WeightedPathTree<'a, G, W>
where
    G: 'a + Graph,
{
    tree: PathTree<'a, G>,
    weight: AHashMap<G::NId, W>,
}

impl<'a, G, W> Tree<'a, G> for WeightedPathTree<'a, G, W>
where
    G: 'a + Graph,
{
    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        self.tree.parent_edge(id)
    }

    fn path_to(&self, target: G::NId) -> Option<Path<'a, G>> {
        self.tree.path_to(target)
    }
}

impl<'a, G, W> WeightedPathTree<'a, G, W>
where
    G: 'a + Graph,
{
    pub fn new(graph: &'a G) -> WeightedPathTree<'a, G, W> {
        WeightedPathTree {
            tree: PathTree::new(graph),
            weight: AHashMap::new(),
        }
    }

    pub fn weight(&self, id: G::NId) -> Option<&W> {
        self.weight.get(&id)
    }

    pub fn contains_node(&self, id: G::NId) -> bool {
        self.tree.contains_node(id)
    }

    pub fn insert_node(&mut self, id: G::NId, parent: Option<G::EId>, weight: W) {
        self.insert_parent(id, parent);
        self.insert_weight(id, weight);
    }

    pub fn insert_parent(&mut self, id: G::NId, parent: Option<G::EId>) {
        self.tree.insert_parent(id, parent);
    }

    pub fn insert_weight(&mut self, id: G::NId, weight: W) {
        self.weight.insert(id, weight);
    }
}
