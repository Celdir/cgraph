use crate::algo::errors::AlgoError;
use crate::algo::shortest_paths::shortest_path_tree::ShortestPathTree;
use crate::graph::edge::Edge;
use crate::graph::traits::Graph;
use std::cmp::Ord;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Add;

type Cycle<'a, G> = Vec<Edge<'a, <G as Graph>::NId, <G as Graph>::EId, <G as Graph>::E>>;

// returns shortest path tree and boolean that is true if negative cycle is found
pub fn bellman_ford<'a, G>(
    graph: &'a G,
    start: G::NId,
) -> Result<(ShortestPathTree<'a, G>, Option<Cycle<'a, G>>), AlgoError>
where
    G: Graph,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    let mut dist = HashMap::new();
    let mut parent = HashMap::new();

    if !graph.contains_node(start) {
        return Err(AlgoError::StartNodeNotFound(format!("{:?}", start)));
    }

    dist.insert(start, G::E::default());

    // n-1 iterations to find shortest paths, +1 final iteration to check for negative cycle
    let (iterations, _) = graph.len();
    let mut change = true;
    let mut last_changed: Option<G::NId> = None;
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
                    last_changed = Some(v_id);
                }
            }
        }
    }

    let cycle = match change {
        true => {
            let mut edges = Vec::new();
            let mut cycle_root_id = last_changed.unwrap();

            // move up to parent N times to ensure cycle_root is in the cycle and not in a chain outside
            // of the cycle.
            // for example: A <--(-3)--> B --1--> C
            // In the above example, C's distance from A might be updated in the last iteration of
            // bellman ford, but C itself is not part of the negative cycle between A and B.
            for _ in 0..iterations {
                cycle_root_id = parent[&cycle_root_id].other(cycle_root_id);
            }

            let mut node_id = cycle_root_id;

            while let Some(edge) = parent.get(&node_id) {
                node_id = edge.other(node_id);
                edges.push(edge.clone());

                if node_id == cycle_root_id {
                    break;
                }
            }

            edges.reverse();
            Some(edges)
        }
        _ => None,
    };

    Ok((ShortestPathTree::new(graph, dist, parent), cycle))
}

#[cfg(test)]
mod tests {
    use crate::algo::errors::AlgoError;
    use crate::algo::shortest_paths::bellman_ford::bellman_ford;
    use crate::graph::traits::{Graph, KeyedGraph, OrdinalGraph, WithCapacity};
    use crate::graph::types::{DiListGraph, UnMapGraph};
    use crate::iter::traits::{Tree};
    use std::matches;

    #[test]
    fn bellman_ford_base_case() {
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

        let (tree, _) = bellman_ford(&graph, "A").unwrap();
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

        let path_to_b = tree.path_to("B").unwrap();
        let ids: Vec<_> = path_to_b.nodes().map(|node| node.id()).collect();
        assert_eq!(ids, vec!["A", "C", "D", "B"]);
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

        let (tree, _) = bellman_ford(&graph, 0).unwrap();
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

        let path_to_e = tree.path_to(4).unwrap();
        let ids: Vec<_> = path_to_e.nodes().map(|node| node.id()).collect();
        assert_eq!(ids, vec![0, 1, 4]);

        let (empty_tree, _) = bellman_ford(&graph, 4).unwrap();
        assert_eq!(empty_tree.dist(0), None);
        assert_eq!(empty_tree.dist(1), None);
        assert_eq!(empty_tree.dist(2), None);
        assert_eq!(empty_tree.dist(3), None);
        assert_eq!(empty_tree.dist(4), Some(&0));
    }

    #[test]
    fn bellman_ford_negative_cycle() {
        // A --5--> B
        // ^       |
        // 2       -6
        // |       v
        // C <(-2)-D
        let mut graph = DiListGraph::with_capacity(4, 4);
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());

        graph.insert_edge(0, 1, 5).expect("nodes should exist");
        graph.insert_edge(1, 3, -6).expect("nodes should exist");
        graph.insert_edge(3, 2, -2).expect("nodes should exist");
        graph.insert_edge(2, 0, 2).expect("nodes should exist");

        let (_, cycle) = bellman_ford(&graph, 0).unwrap();
        assert!(cycle.is_some());
        assert_eq!(cycle.unwrap().len(), 4);
    }

    #[test]
    fn bellman_ford_negative_cycle_of_2() {
        // A --5-- B
        // |       |
        // 2       -4
        // |       |
        // C -(-2)-D
        let mut graph = UnMapGraph::with_capacity(4, 4);
        graph.put_node("A", ());
        graph.put_node("B", ());
        graph.put_node("C", ());
        graph.put_node("D", ());
        graph.insert_edge("A", "B", 5).expect("nodes should exist");
        graph.insert_edge("A", "C", 2).expect("nodes should exist");
        graph.insert_edge("C", "D", -2).expect("nodes should exist");
        graph.insert_edge("B", "D", -4).expect("nodes should exist");

        let (_, cycle) = bellman_ford(&graph, "A").unwrap();
        assert!(cycle.is_some());
        assert_eq!(cycle.unwrap().len(), 2);
    }

    #[test]
    fn bellman_ford_negative_no_cycle() {
        // A-(-5)-> B
        // |        |
        //-2       -6
        // |        v
        // C-(-2)-> D
        let mut graph = DiListGraph::with_capacity(4, 4);
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_edge(0, 1, -5).expect("nodes should exist");
        graph.insert_edge(0, 2, -2).expect("nodes should exist");
        graph.insert_edge(2, 3, -2).expect("nodes should exist");
        graph.insert_edge(1, 3, -6).expect("nodes should exist");

        let (tree, cycle) = bellman_ford(&graph, 0).unwrap();
        assert!(cycle.is_none());
        assert_eq!(tree.dist(0), Some(&0));
        assert_eq!(tree.dist(1), Some(&-5));
        assert_eq!(tree.dist(2), Some(&-2));
        assert_eq!(tree.dist(3), Some(&-11));
    }

    #[test]
    fn bellman_ford_no_start_node_error() {
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

        let result = bellman_ford(&graph, "E");
        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            AlgoError::StartNodeNotFound(..)
        ));
    }
}
