use crate::graph::edge::Edge;
use crate::graph::traits::UndirectedGraph;
use priority_queue::PriorityQueue;
use std::cmp::Ord;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Add;

// Prim's algorithm for minimum spanning trees
// (technically minimum spanning forest as we run Prim's for each connected component)
pub fn mst<'a, G>(graph: &'a G) -> MST<'a, G>
where
    G: UndirectedGraph,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut parent = HashMap::new();
    let mut visited: HashMap<G::NId, bool> = HashMap::new();
    let mut weight = G::E::default();
    let mut connected_components = 0;

    for root in graph.nodes() {
        let root_id = root.id();
        if !visited.contains_key(&root_id) {
            connected_components += 1;

            // initialize min heap
            let mut pq: PriorityQueue<G::NId, Reverse<G::E>> = PriorityQueue::new();
            pq.push(root_id, Reverse(G::E::default()));

            while let Some((id, Reverse(cost))) = pq.pop() {
                visited.insert(id, true);
                weight = weight + cost;

                for (edge, node) in graph.adj(id).unwrap() {
                    let next_cost = edge.data().clone();
                    let nid = node.id();
                    if !visited.contains_key(&nid) {
                        // "push_increase" actually decreases the cost if possible because Reverse
                        let priority = Reverse(next_cost.clone());
                        let old_priority = pq.push_increase(nid, priority);

                        // update parent if next_cost is less than old_cost or old_cost doesn't exist
                        match old_priority {
                            Some(Reverse(old_cost)) if old_cost <= next_cost => {}
                            _ => {
                                parent.insert(nid, edge);
                            }
                        }
                    }
                }
            }
        }
    }

    MST {
        parent,
        weight,
        connected_components,
    }
}

pub struct MST<'a, G>
where
    G: UndirectedGraph,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    parent: HashMap<G::NId, Edge<'a, G::NId, G::EId, G::E>>,
    weight: G::E,
    connected_components: usize,
}

impl<'a, G> MST<'a, G>
where
    G: UndirectedGraph,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    pub fn weight(&self) -> G::E {
        self.weight.clone()
    }

    pub fn connected_components(&self) -> usize {
        self.connected_components
    }

    pub fn parent_edge(&self, id: G::NId) -> Option<Edge<'a, G::NId, G::EId, G::E>> {
        match self.parent.get(&id) {
            Some(edge) => Some(edge.clone()),
            _ => None,
        }
    }

    pub fn ancestor_path(&self, id: G::NId) -> Vec<Edge<'a, G::NId, G::EId, G::E>> {
        let mut edges = Vec::new();
        let mut cur = id;
        while let Some(edge) = self.parent_edge(cur) {
            cur = edge.other(cur);
            edges.push(edge);
        }
        edges.reverse();
        edges
    }
}

#[cfg(test)]
mod tests {
    use crate::algo::mst::mst;
    use crate::graph::mapgraph::MapGraph;

    #[test]
    fn mst_base_case() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let graph = MapGraph::from((
            vec![("A", ()), ("B", ()), ("C", ()), ("D", ())],
            vec![("A", "B", 5), ("A", "C", 2), ("C", "D", 1), ("B", "D", 1)],
        ));

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
        let graph = MapGraph::from((
            vec![
                ("A", ()),
                ("B", ()),
                ("C", ()),
                ("D", ()),
                ("E", ()),
                ("F", ()),
                ("G", ()),
            ],
            vec![
                ("A", "B", 5),
                ("A", "C", 2),
                ("C", "D", 1),
                ("B", "D", 1),
                ("E", "F", 5),
                ("E", "G", 2),
                ("G", "F", 10),
            ],
        ));

        let tree = mst(&graph);
        assert_eq!(tree.weight(), 11);
        assert_eq!(tree.connected_components(), 2);
    }
}
