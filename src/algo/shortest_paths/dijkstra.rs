use crate::algo::errors::AlgoError;
use crate::algo::shortest_paths::shortest_path_tree::ShortestPathTree;
use crate::graph::traits::Graph;
use priority_queue::PriorityQueue;
use std::cmp::Ord;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Add;

pub fn dijkstra<'a, G>(graph: &'a G, start: G::NId) -> Result<ShortestPathTree<'a, G>, AlgoError>
where
    G: Graph,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut dist = HashMap::new();
    let mut parent = HashMap::new();

    if !graph.contains_node(start) {
        return Err(AlgoError::StartNodeNotFound);
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

    Ok(ShortestPathTree::new(dist, parent))
}

#[cfg(test)]
mod tests {
    use crate::algo::errors::AlgoError;
    use crate::algo::shortest_paths::dijkstra::dijkstra;
    use crate::graph::traits::{Graph, KeyedGraph, OrdinalGraph, WithCapacity};
    use crate::graph::types::{DiListGraph, UnMapGraph};

    #[test]
    fn dijkstra_base_case() {
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

        let tree = dijkstra(&graph, "A").unwrap();
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

        let path_to_b = tree.path("B").unwrap();
        let ids: Vec<_> = path_to_b.nodes;
        assert_eq!(ids, vec!["A", "C", "D", "B"]);
    }

    #[test]
    fn dijkstra_directed_base_case() {
        //     > B---\
        //    /  ^    1
        //   5   4     \
        //  /    |      v
        // A -3->C -6-> E
        //  \    ^      ^
        //   1   1     /
        //    \  |    7
        //     > D---/
        //  legend: A-E are actually 0-4 node ids
        let mut graph = DiListGraph::with_capacity(5, 8);
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());

        graph.insert_edge(0, 1, 5).expect("nodes should exist");
        graph.insert_edge(0, 2, 3).expect("nodes should exist");
        graph.insert_edge(0, 3, 1).expect("nodes should exist");
        graph.insert_edge(2, 1, 4).expect("nodes should exist");
        graph.insert_edge(3, 2, 1).expect("nodes should exist");
        graph.insert_edge(1, 4, 1).expect("nodes should exist");
        graph.insert_edge(3, 4, 7).expect("nodes should exist");
        graph.insert_edge(2, 4, 6).expect("nodes should exist");

        let tree = dijkstra(&graph, 0).unwrap();
        assert_eq!(tree.dist(0), Some(&0));
        assert_eq!(tree.dist(1), Some(&5));
        assert_eq!(tree.dist(2), Some(&2));
        assert_eq!(tree.dist(3), Some(&1));
        assert_eq!(tree.dist(4), Some(&6));

        assert!(tree.parent_edge(0).is_none());
        let b_edge = tree.parent_edge(1).unwrap();
        assert_eq!(b_edge.origin(), 0);
        assert_eq!(b_edge.data(), &5);
        let c_edge = tree.parent_edge(2).unwrap();
        assert_eq!(c_edge.origin(), 3);
        assert_eq!(c_edge.data(), &1);
        let d_edge = tree.parent_edge(3).unwrap();
        assert_eq!(d_edge.origin(), 0);
        assert_eq!(d_edge.data(), &1);
        let e_edge = tree.parent_edge(4).unwrap();
        assert_eq!(e_edge.origin(), 1);
        assert_eq!(e_edge.data(), &1);

        let path_to_e = tree.path(4).unwrap();
        let ids: Vec<_> = path_to_e.nodes;
        assert_eq!(ids, vec![0, 1, 4]);

        let empty_tree = dijkstra(&graph, 4).unwrap();
        assert_eq!(empty_tree.dist(0), None);
        assert_eq!(empty_tree.dist(1), None);
        assert_eq!(empty_tree.dist(2), None);
        assert_eq!(empty_tree.dist(3), None);
        assert_eq!(empty_tree.dist(4), Some(&0));
    }

    #[test]
    fn dijkstra_no_start_node_error() {
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

        let result = dijkstra(&graph, "E");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), AlgoError::StartNodeNotFound);
    }
}
