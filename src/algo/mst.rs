use crate::graph::edge::Edge;
use crate::graph::traits::{UndirectedGraph, GraphIter};
use crate::iter::pfs::{pfs, PriorityType};
use crate::iter::traits::{Path, PathTree, Tree};

use std::cmp::Ord;


use std::default::Default;
use std::ops::Add;

// Prim's algorithm for minimum spanning trees
// (technically minimum spanning forest as we run Prim's for each connected component)
pub fn mst<'a, G>(graph: &'a G) -> MST<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut tree = PathTree::new(graph);
    let mut weight = G::E::default();
    let mut connected_components = 0;

    for root in graph.nodes() {
        let root_id = root.id();
        if !tree.contains_node(root_id) {
            connected_components += 1;

            for (edge, node, edge_weight) in pfs(
                graph,
                root_id,
                G::E::default(),
                PriorityType::Min,
                |_, edge, _| edge.data().clone(),
            ) {
                weight = weight + edge_weight;
                tree.insert_parent(node.id(), edge.map(|e| e.id()));
            }
        }
    }

    MST {
        tree,
        weight,
        connected_components,
    }
}

pub struct MST<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    tree: PathTree<'a, G>,
    weight: G::E,
    connected_components: usize,
}

impl<'a, G> Tree<'a, G> for MST<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        self.tree.parent_edge(id)
    }
    fn path_to(&self, target: G::NId) -> Option<Path<'a, G>> {
        self.tree.path_to(target)
    }
}

impl<'a, G> MST<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    pub fn weight(&self) -> G::E {
        self.weight.clone()
    }

    pub fn connected_components(&self) -> usize {
        self.connected_components
    }
}

#[cfg(test)]
mod tests {
    use crate::algo::mst::mst;
    use crate::graph::traits::{KeyedGraph, WithCapacity, GraphMut};
    use crate::graph::types::UnMapGraph;

    #[test]
    fn mst_base_case() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let mut graph = UnMapGraph::with_capacity(4, 4);
        graph.put_node("A", ());
        graph.put_node("B", ());
        graph.put_node("C", ());
        graph.put_node("D", ());
        graph.insert_edge("A", "B", 5).expect("nodes should exist");
        graph.insert_edge("A", "C", 2).expect("nodes should exist");
        graph.insert_edge("C", "D", 1).expect("nodes should exist");
        graph.insert_edge("B", "D", 1).expect("nodes should exist");

        let tree = mst(&graph);
        assert_eq!(tree.weight(), 4);
        assert_eq!(tree.connected_components(), 1);
    }

    #[test]
    fn mst_two_components() {
        // A --5-- B       E -5- F
        // |       |       |    /
        // 2       1       2   10
        // |       |       |  /
        // C --1-- D       G-/
        let mut graph = UnMapGraph::with_capacity(7, 7);
        graph.put_node("A", ());
        graph.put_node("B", ());
        graph.put_node("C", ());
        graph.put_node("D", ());
        graph.put_node("E", ());
        graph.put_node("F", ());
        graph.put_node("G", ());
        graph.insert_edge("A", "B", 5).expect("nodes should exist");
        graph.insert_edge("A", "C", 2).expect("nodes should exist");
        graph.insert_edge("C", "D", 1).expect("nodes should exist");
        graph.insert_edge("B", "D", 1).expect("nodes should exist");
        graph.insert_edge("E", "F", 5).expect("nodes should exist");
        graph.insert_edge("E", "G", 2).expect("nodes should exist");
        graph.insert_edge("G", "F", 10).expect("nodes should exist");

        let tree = mst(&graph);
        assert_eq!(tree.weight(), 11);
        assert_eq!(tree.connected_components(), 2);
    }
}
