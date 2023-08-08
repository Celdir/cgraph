use crate::graph::edge::Edge;
use crate::graph::traits::Graph;
use crate::iter::dfs::dfs;
use std::cmp::min;
use std::collections::HashMap;

pub fn bridges<'a, G>(graph: &'a G) -> Vec<Edge<'a, G::NId, G::EId, G::E>>
where
    G: Graph,
{
    let mut order = HashMap::new();
    let mut low = HashMap::new();
    let mut time = 0;
    for root in graph.nodes() {
        let root_id = root.id();
        if !order.contains_key(&root_id) {
            let dfs: Vec<_> = dfs(graph, root_id).collect();
            for (_, node) in &dfs {
                order.insert(node.id(), time);
                time += 1;
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

    graph
        .edges()
        .filter(|edge| low[&edge.v()] > order[&edge.u()])
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::algo::bridges::bridges;
    use crate::graph::traits::OrdinalGraph;
    use crate::graph::types::UnListGraph;

    #[test]
    fn square_no_bridges() {
        // N0 ----- N1
        // |         |
        // |         |
        // |         |
        // N2 ----- N3
        let mut graph = UnListGraph::builder()
            .with_size(4)
            .edge(0, 1, ())
            .edge(0, 2, ())
            .edge(2, 3, ())
            .edge(1, 3, ())
            .build();
        assert_eq!(bridges(&graph).len(), 0);
    }

    #[test]
    fn two_bridges() {
        // N0 ----- N1 ----- N4
        // |         |
        // |         |
        // |         |
        // N2 ----- N3 ----- N5
        let mut graph = UnListGraph::builder()
            .with_size(6)
            .edge(0, 1, ())
            .edge(0, 2, ())
            .edge(2, 3, ())
            .edge(1, 3, ())
            .edge(1, 4, ())
            .edge(3, 5, ())
            .build();
        let bridges = bridges(&graph);
        assert_eq!(bridges.len(), 2);
        assert_eq!(bridges[0].id(), 4);
        assert_eq!(bridges[1].id(), 5);
    }
}
