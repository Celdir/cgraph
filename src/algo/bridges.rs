use crate::graph::edge::Edge;
use crate::graph::node::Node;
use crate::graph::traits::{GraphIter, UndirectedGraph};
use crate::graph::types::NodeHashMap;
use crate::iter::dfs::dfs;

use std::cmp::min;

pub fn bridges<'a, G>(graph: &'a G) -> Vec<Edge<'a, G::NId, G::EId, G::E>>
where
    G: UndirectedGraph + GraphIter,
{
    let dp = dp(graph);
    graph
        .edges()
        .filter(|edge| {
            if dp.order[&edge.v()] > dp.order[&edge.u()] {
                dp.low[&edge.v()] > dp.order[&edge.u()]
            } else {
                dp.low[&edge.u()] > dp.order[&edge.v()]
            }
        })
        .collect()
}

pub fn articulation_points<'a, G>(graph: &'a G) -> Vec<Node<'a, G::NId, G::N>>
where
    G: UndirectedGraph + GraphIter,
{
    let dp = dp(graph);
    graph
        .nodes()
        .filter(|node| {
            let id = node.id();
            if dp.roots.contains_key(&id) {
                dp.roots[&id] > 1
            } else {
                for (_, neighbor) in graph.adj(id).unwrap() {
                    let nid = neighbor.id();
                    if dp.order[&nid] > dp.order[&id] && dp.low[&nid] >= dp.order[&id] {
                        return true;
                    }
                }
                false
            }
        })
        .collect()
}

fn dp<'a, G>(graph: &'a G) -> DP<G>
where
    G: UndirectedGraph + GraphIter,
{
    let mut order = NodeHashMap::<G, usize>::default();
    let mut low = NodeHashMap::<G, usize>::default();
    let mut roots = NodeHashMap::<G, usize>::default(); // maps root id to number of children
    let mut time = 0;
    for root in graph.nodes() {
        let root_id = root.id();
        if !order.contains_key(&root_id) {
            roots.insert(root_id, 0);

            let dfs: Vec<_> = dfs(graph, root_id).collect();
            for (edge, node) in &dfs {
                order.insert(node.id(), time);
                time += 1;
                if edge.is_some() && edge.as_ref().unwrap().other(node.id()) == root_id {
                    roots.insert(root_id, roots[&root_id] + 1);
                }
            }

            for (parent_edge, node) in dfs.iter().rev() {
                let id = node.id();
                low.insert(id, order[&id]);
                for (edge, neighbor) in graph.adj(node.id()).unwrap() {
                    let nid = neighbor.id();
                    if parent_edge.is_some() && edge.id() == parent_edge.as_ref().unwrap().id() {
                        continue;
                    }
                    if order[&nid] < order[&id] {
                        low.insert(id, min(low[&id], order[&nid]));
                    } else {
                        low.insert(id, min(low[&id], low[&nid]));
                    }
                }
            }
        }
    }
    DP { order, low, roots }
}

struct DP<G>
where
    G: UndirectedGraph + GraphIter,
{
    order: NodeHashMap<G, usize>,
    low: NodeHashMap<G, usize>,
    roots: NodeHashMap<G, usize>,
}

#[cfg(test)]
mod tests {
    use crate::algo::bridges::{articulation_points, bridges};
    use crate::graph::traits::OrdinalGraph;
    use crate::graph::types::UnListGraph;

    #[test]
    fn square_no_bridges() {
        // N0 ----- N1
        // |         |
        // |         |
        // |         |
        // N2 ----- N3
        let graph = UnListGraph::builder()
            .with_size(4)
            .edge(0, 1, ())
            .edge(0, 2, ())
            .edge(2, 3, ())
            .edge(1, 3, ())
            .build();
        assert_eq!(bridges(&graph).len(), 0);
    }

    #[test]
    fn square_no_articulation() {
        // N0 ----- N1
        // |         |
        // |         |
        // |         |
        // N2 ----- N3
        let graph = UnListGraph::builder()
            .with_size(4)
            .edge(0, 1, ())
            .edge(0, 2, ())
            .edge(2, 3, ())
            .edge(1, 3, ())
            .build();
        assert_eq!(articulation_points(&graph).len(), 0);
    }

    #[test]
    fn two_articulation_points() {
        // N0 ----- N1 ----- N4
        // |         |
        // |         |
        // |         |
        // N2 ----- N3 ----- N5
        let graph = UnListGraph::builder()
            .with_size(6)
            .edge(0, 1, ())
            .edge(0, 2, ())
            .edge(2, 3, ())
            .edge(1, 3, ())
            .edge(1, 4, ())
            .edge(5, 3, ())
            .build();
        let points = articulation_points(&graph);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].id(), 1);
        assert_eq!(points[1].id(), 3);
    }

    #[test]
    fn one_articulation_point() {
        // N0 ----- N1 ----- N3
        // |      / |      /
        // |   __/  |   __/
        // | _/     | _/
        // N2       N4
        let graph = UnListGraph::builder()
            .with_size(5)
            .edge(0, 1, ())
            .edge(0, 2, ())
            .edge(2, 1, ())
            .edge(1, 3, ())
            .edge(1, 4, ())
            .edge(4, 3, ())
            .build();
        let points = articulation_points(&graph);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].id(), 1);
    }
}
