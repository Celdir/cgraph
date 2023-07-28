use crate::algo::errors::AlgoError;
use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;
use crate::iter::pfs::{pfs, Pfs, PriorityType};
use crate::iter::traits::Traversal;
use priority_queue::PriorityQueue;
use std::cmp::Ord;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::default::Default;
use std::ops::Add;

pub fn astar<'a, G>(
    graph: &'a G,
    start: G::NId,
    heuristic: impl Fn(&Node<'a, G::NId, G::N>) -> G::E,
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
        move |dist, edge, node| dist + edge.data().clone() + heuristic(node),
    ))
}

#[cfg(test)]
mod tests {
    use crate::algo::shortest_paths::astar::astar;
    use crate::graph::traits::{Graph, KeyedGraph};
    use crate::graph::types::UnMapGraph;
    use crate::iter::traits::Traversal;

    #[test]
    fn astar_grid() {
        let mut graph = UnMapGraph::<(usize, usize), (), i64>::new();

        for x in 0..101 {
            for y in 0..101 {
                graph.put_node((x, y), ());
            }
        }

        let deltas = vec![(0, 1), (1, 0)];

        for x in 0..101 {
            for y in 0..101 {
                for d in &deltas {
                    let nx = x + d.0;
                    let ny = y + d.1;
                    if nx < 101 && ny < 101 {
                        graph
                            .insert_edge((x, y), (nx, ny), 1)
                            .expect("nodes should exist");
                    }
                }
            }
        }

        let (_, _, dist) = astar(&graph, (0, 0), |node| {
            let (x, y) = node.id();
            (((100 - x).pow(2) + (100 - y).pow(2)) as f64)
                .sqrt()
                .floor() as i64
        })
        .expect("start node should exist")
        .find(|(_, node, _)| node.id() == (100, 100))
        .expect("end ndoe should be found");

        assert_eq!(dist, 200);
    }
}
