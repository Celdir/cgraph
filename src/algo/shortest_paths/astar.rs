use crate::algo::errors::AlgoError;
use crate::algo::shortest_paths::shortest_path_tree::{ShortestPath, ShortestPathTree};
use crate::graph::node::Node;
use crate::graph::traits::Graph;
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
) -> Result<ShortestPath<'a, G>, AlgoError>
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

    Ok(ShortestPathTree::new(dist, parent)
        .path(end)
        .ok_or(AlgoError::NoPathFromStartToEnd)?)
}

#[cfg(test)]
mod tests {
    use crate::algo::shortest_paths::astar::astar;
    use crate::graph::traits::{Graph, KeyedGraph};
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

        let path = astar(&graph, (0, 0), (100, 100), |node| {
            let (x, y) = node.id();
            (((100 - x).pow(2) + (100 - y).pow(2)) as f64)
                .sqrt()
                .floor() as i64
        })
        .expect("path should exist");

        assert_eq!(path.dist, 200);
    }
}
