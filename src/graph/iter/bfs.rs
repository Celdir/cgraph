/*use crate::graph::structure::{Graph};
use std::collections::{HashMap, VecDeque};

pub struct Bfs<'a, G: Graph<'a>> {
    queue: VecDeque<G::Idx>,
    parent: HashMap<G::Idx, Option<(G::Idx, &'a G::Edge)>>,
    graph: &'a G,
    roots_itr: Option<Box<dyn Iterator<Item = (G::Idx, &'a G::Node)> + 'a>>,
}

impl<'a, G: Graph<'a>> Iterator for Bfs<'a, G> {
    type Item = (G::Idx, &'a G::Node);

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            let (next_root, _) = self.roots_itr.as_mut()?.next()?;
            self.queue.push_back(next_root);
        }

        let cur = self.queue.pop_front()?;
        if !self.parent.contains_key(&cur) {
            self.parent.insert(cur, None);
        }

        for (_, v, e) in self.graph.adj(cur) {
            if !self.parent.contains_key(&v) {
                self.parent.insert(v, Some((cur, e)));
                self.queue.push_back(v);
            }
        }
        Some((cur, self.graph.get_node(cur)?))
    }
}

impl<'a, G: Graph<'a>> Bfs<'a, G> {
    fn new(graph: &'a G, start: Option<G::Idx>) -> Bfs<'a, G> {
        match start {
            Some(idx) => Bfs {
                graph,
                queue: VecDeque::from(vec![idx,]),
                parent: HashMap::new(),
                roots_itr: None,
            },
            None => Bfs{
                graph,
                queue: VecDeque::new(),
                parent: HashMap::new(),
                roots_itr: Some(graph.nodes()),
            },
        }
    }
}

pub trait BfsTrait<'a, G: Graph<'a>> {
    fn bfs(&'a self) -> Bfs<'a, G>;
    fn bfs_from(&'a self, start: G::Idx) -> Bfs<'a, G>;
}

impl<'a, G: Graph<'a>> BfsTrait<'a, G> for G {
    fn bfs(&'a self) -> Bfs<'a, G> {
        Bfs::new(&self, None)
    }

    fn bfs_from(&'a self, start: G::Idx) -> Bfs<'a, G> {
        Bfs::new(&self, Some(start))
    }
}

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
