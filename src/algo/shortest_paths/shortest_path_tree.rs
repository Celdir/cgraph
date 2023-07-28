use crate::graph::edge::Edge;
use crate::graph::traits::Graph;
use crate::iter::traits::{Path, Tree};
use std::collections::HashMap;

pub struct ShortestPathTree<'a, G>
where
    G: Graph,
    G::E: Clone,
{
    graph: &'a G,
    dist: HashMap<G::NId, G::E>,
    parent: HashMap<G::NId, Edge<'a, G::NId, G::EId, G::E>>,
}

impl<'a, G> Tree<'a, G> for ShortestPathTree<'a, G>
where
    G: 'a + Graph,
    G::E: Clone,
{
    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        match self.parent.get(&id) {
            Some(edge) => Some(edge.clone()),
            _ => None,
        }
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

impl<'a, G> ShortestPathTree<'a, G>
where
    G: 'a + Graph,
    G::E: Clone,
{
    pub fn new(
        graph: &'a G,
        dist: HashMap<G::NId, G::E>,
        parent: HashMap<G::NId, Edge<'a, G::NId, G::EId, G::E>>,
    ) -> ShortestPathTree<'a, G> {
        ShortestPathTree { graph, dist, parent }
    }

    pub fn dist(&self, id: G::NId) -> Option<&G::E> {
        self.dist.get(&id)
    }
}
