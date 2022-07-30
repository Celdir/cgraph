use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::DirectedGraph;
use crate::graph::structure::node::Node;
use std::collections::{HashMap, VecDeque};

// assumes the graph has an edge from A to B if B depends on A
pub fn topological_sort<'a, G: DirectedGraph<'a>>(graph: &'a G) -> Option<Vec<Node<'a, G::NId, G::N>>> {
    let mut in_degree: HashMap<G::NId, usize> = HashMap::new();
    let mut queue: VecDeque<Node<'a, G::NId, G::N>> = VecDeque::new();

    for node in graph.nodes() {
        let id = node.id();
        let deg = graph.in_degree(id);

        in_degree.insert(id, deg);
        if deg == 0 {
            queue.push_back(node);
        }
    }

    let mut order: Vec<Node<'a, G::NId, G::N>> = Vec::new();
    while !queue.is_empty() {
        let node = queue.pop_front()?;

        for (_, next) in graph.out_edges(node.id())? {
            in_degree.entry(next.id()).and_modify(|deg| {*deg -= 1} );
            if in_degree[&next.id()] == 0 {
                queue.push_back(next);
            }
        }

        order.push(node);
    }

    let (num_nodes, _) = graph.len();
    if order.len() != num_nodes {
        // There must be a cycle in the graph and therefore no topological order
        return None
    }

    Some(order)
}

#[cfg(test)]
mod tests {
    use crate::graph::structure::vecgraph::StableVecGraph;
    use crate::graph::algo::topological_sort::topological_sort;

    #[test]
    fn top_sort_triangle() {
        let graph = StableVecGraph::from((
            vec![(); 3],
            vec![
                (0, 1, ()),
                (0, 2, ()),
                (1, 2, ()),
            ],
        ));

        let expected_order = vec![0, 1, 2];
        let order: Vec<_> = topological_sort(&graph).unwrap().iter().map(|node| node.id()).collect();
        assert_eq!(expected_order, order);
    }
}
