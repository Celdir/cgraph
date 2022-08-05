use crate::graph::algo::shortest_paths::shortest_path_tree::ShortestPathTree;
use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::Graph;
use crate::graph::structure::node::Node;
use priority_queue::PriorityQueue;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Add;

pub fn dijkstra<'a, G>(graph: &'a G, start: G::NId) -> ShortestPathTree<'a, G>
where
    G: Graph<'a>,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut dist = HashMap::new();
    let mut parent = HashMap::new();

    if !graph.contains_node(start) {
        return ShortestPathTree::new(dist, parent);
    }

    // initialize min heap
    let mut pq: PriorityQueue<G::NId, Reverse<G::E>> = PriorityQueue::new();
    pq.push(start, Reverse(G::E::default()));

    while let Some((id, Reverse(cost))) = pq.pop() {
        dist.insert(id, cost.clone());

        for (edge, node) in graph.adj(id).unwrap() {
            let next_cost = cost.clone() + edge.data().clone();
            let nid = node.id();
            if !dist.contains_key(&nid) {
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

    ShortestPathTree::new(dist, parent)
}

#[cfg(test)]
mod tests {
    use crate::graph::algo::shortest_paths::dijkstra::dijkstra;
    use crate::graph::structure::mapgraph::MapGraph;

    #[test]
    fn dijkstra_base_case() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let graph = MapGraph::from((
            vec![("A", ()), ("B", ()), ("C", ()), ("D", ())],
            vec![("A", "B", 5), ("A", "C", 2), ("C", "D", 1), ("B", "D", 1)],
        ));

        let tree = dijkstra(&graph, "A");
        assert_eq!(tree.dist("A"), Some(&0));
        assert_eq!(tree.dist("B"), Some(&4));
        assert_eq!(tree.dist("C"), Some(&2));
        assert_eq!(tree.dist("D"), Some(&3));

        assert!(tree.parent_edge("A").is_none());
        let b_edge = tree.parent_edge("B").unwrap();
        assert_eq!(b_edge.other("B"), "D");
        assert_eq!(b_edge.data(), &1);
        let c_edge = tree.parent_edge("C").unwrap();
        assert_eq!(c_edge.other("C"), "A");
        assert_eq!(c_edge.data(), &2);
        let d_edge = tree.parent_edge("D").unwrap();
        assert_eq!(d_edge.other("D"), "C");
        assert_eq!(d_edge.data(), &1);

        let path_to_b = tree.path("B");
        let ids: Vec<_> = path_to_b.iter().map(|edge| edge.origin()).collect();
        assert_eq!(ids, vec!["A", "C", "D"]);
    }
}
