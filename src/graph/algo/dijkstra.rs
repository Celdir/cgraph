use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::Graph;
use crate::graph::structure::node::Node;
use priority_queue::PriorityQueue;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};
use std::default::Default;
use std::ops::Add;

// assumes the graph has an edge from A to B if B depends on A
pub fn dijkstra<'a, G>(graph: &'a G, start: G::NId) -> HashMap<G::NId, G::E>
where
    G: Graph<'a>,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut dist = HashMap::new();

    if !graph.contains_node(start) {
        return dist;
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
                pq.push_increase(nid, Reverse(next_cost));
            }
        }
    }

    dist
}

/*
#[cfg(test)]
mod tests {
    use crate::graph::algo::topological_sort::topological_sort;
    use crate::graph::structure::vecgraph::StableVecGraph;

    #[test]
    fn top_sort_triangle() {
        let graph = StableVecGraph::from((vec![(); 3], vec![(0, 1, ()), (0, 2, ()), (1, 2, ())]));

        let expected_order = vec![0, 1, 2];
        let order: Vec<_> = topological_sort(&graph)
            .unwrap()
            .iter()
            .map(|node| node.id())
            .collect();
        assert_eq!(expected_order, order);
    }

    #[test]
    fn top_sort_cycle() {
        let graph = StableVecGraph::from((vec![(); 3], vec![(0, 1, ()), (1, 2, ()), (2, 0, ())]));

        let order: Option<Vec<_>> = topological_sort(&graph);
        assert!(order.is_none());
    }
}*/
