use crate::graph::algo::shortest_paths::shortest_path_tree::{ShortestPathTree, ShortestPath};
use crate::graph::structure::graph::Graph;
use crate::graph::structure::node::Node;
use priority_queue::PriorityQueue;
use std::cmp::Ord;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Add;

pub fn astar<'a, G>(
    graph: &'a G,
    start: G::NId,
    end: G::NId,
    heuristic: impl Fn(Node<'a, G::NId, G::N>) -> G::E,
) -> Option<ShortestPath<'a, G>>
where
    G: Graph<'a>,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut dist = HashMap::new();
    let mut parent = HashMap::new();

    if !graph.contains_node(start) {
        return None;
    }

    // initialize min heap
    let mut fringe: PriorityQueue<G::NId, Reverse<G::E>> = PriorityQueue::new();
    fringe.push(
        start,
        Reverse(G::E::default() + heuristic(graph.node(start).unwrap())),
    );
    dist.insert(start, G::E::default());

    while let Some((id, _)) = fringe.pop() {
        for (edge, node) in graph.adj(id).unwrap() {
            let next_dist = dist[&id].clone() + edge.data().clone();
            let nid = node.id();
            if !dist.contains_key(&nid) || next_dist < dist[&nid] {
                dist.insert(nid, next_dist.clone());
                parent.insert(nid, edge);

                // "push_increase" actually decreases the cost if possible because Reverse
                let next_cost = next_dist + heuristic(node);
                let priority = Reverse(next_cost.clone());
                fringe.push_increase(nid, priority);
            }
        }
    }

    ShortestPathTree::new(dist, parent).path(end)
}

/*
#[cfg(test)]
mod tests {
    use crate::graph::algo::shortest_paths::astar::astar;
    use crate::graph::structure::mapgraph::MapGraph;
    use crate::graph::structure::vecgraph::StableVecGraph;

    #[test]
    fn astar_base_case() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let graph = MapGraph::from((
            vec![("A", ()), ("B", ()), ("C", ()), ("D", ())],
            vec![("A", "B", 5), ("A", "C", 2), ("C", "D", 1), ("B", "D", 1)],
        ));

        let tree = astar(&graph, "A");
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

    #[test]
    fn astar_directed_base_case() {
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
        let graph = StableVecGraph::from((
            vec![(); 5],
            vec![
                (0, 1, 5),
                (0, 2, 3),
                (0, 3, 1),
                (2, 1, 4),
                (3, 2, 1),
                (1, 4, 1),
                (3, 4, 7),
                (2, 4, 6),
            ],
        ));

        let tree = astar(&graph, 0);
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

        let path_to_e = tree.path(4);
        let ids: Vec<_> = path_to_e.iter().map(|edge| edge.origin()).collect();
        assert_eq!(ids, vec![0, 1]);

        let empty_tree = astar(&graph, 4);
        assert_eq!(empty_tree.dist(0), None);
        assert_eq!(empty_tree.dist(1), None);
        assert_eq!(empty_tree.dist(2), None);
        assert_eq!(empty_tree.dist(3), None);
        assert_eq!(empty_tree.dist(4), Some(&0));
    }
}*/
