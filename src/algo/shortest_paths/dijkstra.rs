use crate::algo::errors::AlgoError;
use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;
use crate::iter::pfs::{pfs, Pfs, PriorityType};

use std::cmp::Ord;

use std::default::Default;
use std::ops::Add;

pub fn dijkstra<'a, G>(
    graph: &'a G,
    start: G::NId,
) -> Result<
    Pfs<
        G,
        G::E,
        impl Fn(G::E, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> G::E,
        impl Fn(&Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> bool,
    >,
    AlgoError,
>
where
    G: Graph,
    G::E: Add<Output = G::E> + Ord + Default + Clone,
{
    if !graph.contains_node(start) {
        return Err(AlgoError::StartNodeNotFound(format!("{:?}", start)));
    }
    Ok(pfs(
        graph,
        start,
        G::E::default(),
        PriorityType::Min,
        |dist, edge, _| dist + edge.data().clone(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::algo::errors::AlgoError;
    use crate::algo::shortest_paths::dijkstra::dijkstra;
    use crate::graph::traits::{Graph, GraphMut, KeyedGraph, OrdinalGraph, WithCapacity};
    use crate::graph::types::{DiListGraph, UnMapGraph};
    use crate::iter::traits::{Tree, WeightedPathTree};
    use std::matches;

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

        let tree = WeightedPathTree::from(dijkstra(&graph, "A").unwrap());
        assert_eq!(tree.weight("A"), Some(&0));
        assert_eq!(tree.weight("B"), Some(&4));
        assert_eq!(tree.weight("C"), Some(&2));
        assert_eq!(tree.weight("D"), Some(&3));

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

        let tree = WeightedPathTree::from(dijkstra(&graph, 0).unwrap());
        assert_eq!(tree.weight(0), Some(&0));
        assert_eq!(tree.weight(1), Some(&5));
        assert_eq!(tree.weight(2), Some(&2));
        assert_eq!(tree.weight(3), Some(&1));
        assert_eq!(tree.weight(4), Some(&6));

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

        let empty_tree = WeightedPathTree::from(dijkstra(&graph, 4).unwrap());
        assert_eq!(empty_tree.weight(0), None);
        assert_eq!(empty_tree.weight(1), None);
        assert_eq!(empty_tree.weight(2), None);
        assert_eq!(empty_tree.weight(3), None);
        assert_eq!(empty_tree.weight(4), Some(&0));
    }

    #[test]
    fn dijkstra_no_start_node_empty() {
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
        assert!(matches!(
            result.err().unwrap(),
            AlgoError::StartNodeNotFound(..)
        ));
    }
}
