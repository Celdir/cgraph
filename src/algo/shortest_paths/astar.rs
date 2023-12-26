use crate::algo::errors::AlgoError;
use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::Graph;
use crate::iter::pfs::{pfs, Pfs, PriorityType};
use std::cmp::Ord;
use std::cmp::Ordering;
use std::default::Default;
use std::ops::Add;

pub fn astar<'a, G>(
    graph: &'a G,
    start: G::NId,
    heuristic: impl Fn(&Node<'a, G::NId, G::N>) -> G::E,
) -> Result<
    Pfs<
        G,
        Weight<G::E>,
        impl Fn(Weight<G::E>, &Edge<'a, G::NId, G::EId, G::E>, &Node<'a, G::NId, G::N>) -> Weight<G::E>,
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
        Weight {
            distance: G::E::default(),
            priority: G::E::default(),
        },
        PriorityType::Min,
        move |acc: Weight<G::E>, edge, node| {
            let distance = acc.distance + edge.data().clone();
            let priority = acc.priority + edge.data().clone() + heuristic(node);
            Weight { distance, priority }
        },
    ))
}

#[derive(PartialEq, Eq, Clone)]
pub struct Weight<T>
where
    T: Add<Output = T> + Ord + Default + Clone,
{
    pub distance: T,
    pub priority: T,
}

impl<T> Ord for Weight<T>
where
    T: Add<Output = T> + Ord + Default + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl<T> PartialOrd for Weight<T>
where
    T: Add<Output = T> + Ord + Default + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::algo::shortest_paths::astar::astar;
    use crate::graph::traits::{Graph, KeyedGraph, GraphMut};
    use crate::graph::types::UnMapGraph;
    

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

        let (_, _, weight) = astar(&graph, (0, 0), |node| {
            let (x, y) = node.id();
            (((100 - x).pow(2) + (100 - y).pow(2)) as f64)
                .sqrt()
                .floor() as i64
        })
        .expect("start node should exist")
        .find(|(_, node, _)| node.id() == (100, 100))
        .expect("end node should be found");

        assert_eq!(weight.distance, 200);
    }
}
