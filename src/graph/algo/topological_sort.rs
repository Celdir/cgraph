use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::DirectedGraph;
use crate::graph::structure::node::Node;
use std::collections::{HashMap, VecDeque};

pub fn topological_order<'a, G: DirectedGraph<'a>>(graph: &'a G) -> Option<Vec<Node<'a, G::NId, G::N>>> {
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

    Some(order)
}
