use crate::graph::algo::shortest_paths::shortest_path_tree::ShortestPathTree;
use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::Graph;
use crate::graph::structure::node::Node;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Add;

// TODO: add cycle finding function to this file

// returns shortest path tree and boolean that is true if negative cycle is found
pub fn bellman_ford<'a, G>(graph: &'a G, start: G::NId) -> (ShortestPathTree<'a, G>, bool)
where
    G: Graph<'a>,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut dist = HashMap::new();
    let mut parent = HashMap::new();

    if !graph.contains_node(start) {
        return (ShortestPathTree::new(dist, parent), false);
    }

    dist.insert(start, G::E::default());

    // n-1 iterations to find shortest paths, +1 final iteration to check for negative cycle
    let (iterations, _) = graph.len();
    let mut change = true;
    for _ in 0..iterations {
        if !change {
            break;
        }
        change = false;

        for u in graph.nodes() {
            if !dist.contains_key(&u.id()) {
                continue;
            }
            for (edge, v) in graph.adj(u.id()).unwrap() {
                let v_id = v.id();
                let weight = edge.data().clone();

                let u_dist = dist[&u.id()].clone();
                let v_dist_new = u_dist + weight;

                if !dist.contains_key(&v_id) || v_dist_new < dist[&v_id] {
                    dist.insert(v_id, v_dist_new);
                    parent.insert(v_id, edge);
                    change = true;
                }
            }
        }
    }

    (ShortestPathTree::new(dist, parent), change)
}

#[cfg(test)]
mod tests {
    use crate::graph::algo::shortest_paths::bellman_ford::bellman_ford;
    use crate::graph::structure::mapgraph::MapGraph;
    use crate::graph::structure::vecgraph::StableVecGraph;

    #[test]
    fn bellman_ford_base_case() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let graph = MapGraph::from((
            vec![("A", ()), ("B", ()), ("C", ()), ("D", ())],
            vec![("A", "B", 5), ("A", "C", 2), ("C", "D", 1), ("B", "D", 1)],
        ));

        let (tree, _) = bellman_ford(&graph, "A");
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
    fn bellman_ford_directed_base_case() {
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

        let (tree, _) = bellman_ford(&graph, 0);
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

        let (empty_tree, _) = bellman_ford(&graph, 4);
        assert_eq!(empty_tree.dist(0), None);
        assert_eq!(empty_tree.dist(1), None);
        assert_eq!(empty_tree.dist(2), None);
        assert_eq!(empty_tree.dist(3), None);
        assert_eq!(empty_tree.dist(4), Some(&0));
    }

    #[test]
    fn bellman_ford_negative_cycle() {
        // A --5-- B
        // |       |
        // 2       -6
        // |       |
        // C -(-2)-D
        let graph = MapGraph::from((
            vec![("A", ()), ("B", ()), ("C", ()), ("D", ())],
            vec![("A", "B", 5), ("A", "C", 2), ("C", "D", -2), ("B", "D", -6)],
        ));

        let (tree, has_cycle) = bellman_ford(&graph, "A");
        assert!(has_cycle);
    }

    #[test]
    fn bellman_ford_negative_cycle_of_2() {
        // A --5-- B
        // |       |
        // 2       -4
        // |       |
        // C -(-2)-D
        let graph = MapGraph::from((
            vec![("A", ()), ("B", ()), ("C", ()), ("D", ())],
            vec![("A", "B", 5), ("A", "C", 2), ("C", "D", -2), ("B", "D", -4)],
        ));

        let (tree, has_cycle) = bellman_ford(&graph, "A");
        assert!(has_cycle);
    }

    #[test]
    fn bellman_ford_negative_no_cycle() {
        // A-(-5)-> B
        // |        |
        //-2       -6
        // |        v
        // C-(-2)-> D
        let graph = StableVecGraph::from((
            vec![(); 4],
            vec![(0, 1, -5), (0, 2, -2), (2, 3, -2), (1, 3, -6)],
        ));

        let (tree, has_cycle) = bellman_ford(&graph, 0);
        assert!(!has_cycle);
        assert_eq!(tree.dist(0), Some(&0));
        assert_eq!(tree.dist(1), Some(&-5));
        assert_eq!(tree.dist(2), Some(&-2));
        assert_eq!(tree.dist(3), Some(&-11));
    }
}