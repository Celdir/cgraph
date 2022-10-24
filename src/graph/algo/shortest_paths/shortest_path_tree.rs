use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::Graph;
use std::collections::HashMap;

pub struct ShortestPathTree<'a, G>
where
    G: Graph,
    G::E: Clone,
{
    dist: HashMap<G::NId, G::E>,
    parent: HashMap<G::NId, Edge<'a, G::NId, G::EId, G::E>>,
}

impl<'a, G> ShortestPathTree<'a, G>
where
    G: Graph,
    G::E: Clone,
{
    pub fn new(
        dist: HashMap<G::NId, G::E>,
        parent: HashMap<G::NId, Edge<'a, G::NId, G::EId, G::E>>,
    ) -> ShortestPathTree<'a, G> {
        ShortestPathTree { dist, parent }
    }

    pub fn dist(&self, id: G::NId) -> Option<&G::E> {
        self.dist.get(&id)
    }

    pub fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        match self.parent.get(&id) {
            Some(edge) => Some(edge.clone()),
            _ => None,
        }
    }

    pub fn path(&self, id: G::NId) -> Option<ShortestPath<'a, G>> {
        let dist = self.dist(id)?.clone();

        let mut edges = Vec::new();
        let mut cur = id;
        while let Some(edge) = self.parent_edge(cur) {
            cur = edge.other(cur);
            edges.push(edge);
        }
        edges.reverse();

        Some(ShortestPath { dist, edges })
    }
}

pub struct ShortestPath<'a, G>
where
    G: Graph,
    G::E: Clone,
{
    pub dist: G::E,
    pub edges: Vec<Edge<'a, G::NId, G::EId, G::E>>,
}
