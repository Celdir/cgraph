use crate::graph::structure::graph::{Graph};
use crate::graph::structure::node::{Node};
use crate::graph::structure::edge::{Edge};
use std::collections::{HashMap, VecDeque};

pub fn bfs<'a, G: Graph<'a>>(graph: &'a G, start: <G as Graph<'a>>::NId) -> Bfs<'a, G> {
    Bfs::new(graph, start)
}

pub struct Bfs<'a, G: Graph<'a>> {
    graph: &'a G,
    queue: VecDeque<<G as Graph<'a>>::NId>,
    parent: HashMap<<G as Graph<'a>>::NId, Option<<G as Graph<'a>>::EId>>,
}

impl<'a, G: Graph<'a>> Iterator for Bfs<'a, G> {
    type Item = (Option<Edge<'a, <G as Graph<'a>>::NId, <G as Graph<'a>>::EId, <G as Graph<'a>>::E>>, Node<'a, <G as Graph<'a>>::NId, <G as Graph<'a>>::N>);

    fn next(&mut self) -> Option<Self::Item> {
        let node_id = self.queue.pop_front()?;
        if !self.parent.contains_key(&node_id) {
            self.parent.insert(node_id, None);
        }

        for (edge, node) in self.graph.adj(node_id)? {
            let next_id = node.id();
            if !self.parent.contains_key(&next_id) {
                self.parent.insert(next_id, Some(edge.id()));
                self.queue.push_back(next_id);
            }
        }

        let node = self.graph.node(node_id).unwrap();
        let parent_edge_opt = self.parent.get(&node_id).unwrap().map(|edge_id| self.graph.edge(edge_id).unwrap());
        Some((parent_edge_opt, node))
    }
}

impl<'a, G: Graph<'a>> Bfs<'a, G> {
    fn new(graph: &'a G, start: <G as Graph<'a>>::NId) -> Bfs<'a, G> {
        Bfs{
            graph,
            queue: VecDeque::from(vec![start,]),
            parent: HashMap::new(),
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::graph::structure::{SimpleGraph};
    use crate::graph::iter::bfs::{BfsTrait};

    #[test]
    fn bfs_from_base_case() {
        let graph = SimpleGraph::from(vec![(0, 1), (0, 2), (1, 3), (1, 4), (2, 5), (2, 6)]);

        let expected_order = vec![0, 1, 2, 3, 4, 5, 6];
        let bfs: Vec<_> = graph.bfs_from(0).map(|i| i.0).collect();
        assert_eq!(&expected_order, &bfs);
    }

    #[test]
    fn bfs_from_dag() {
        let graph = SimpleGraph::from(vec![(0, 1), (0, 2), (1, 2), (1, 3), (1, 4), (2, 3), (2, 5), (2, 6)]);

        let expected_order = vec![0, 1, 2, 3, 4, 5, 6];
        let bfs: Vec<_> = graph.bfs_from(0).map(|i| i.0).collect();
        assert_eq!(&expected_order, &bfs);
    }
}*/
