use crate::graph::traits::{GraphIter, UndirectedGraph};
use crate::graph::types::NodeHashMap;
use crate::iter::bfs::bfs;

// Returns map of node ids to component id, where component ids count up from 0
pub fn connected_components<G: UndirectedGraph + GraphIter>(graph: &G) -> NodeHashMap<G, usize> {
    let mut component_id = 0;
    let mut component = NodeHashMap::<G, usize>::default();

    for start in graph.nodes() {
        if !component.contains_key(&start.id()) {
            for (_, node) in bfs(graph, start.id()) {
                component.insert(node.id(), component_id);
            }
            component_id += 1;
        }
    }

    component
}

#[cfg(test)]
mod tests {
    use crate::algo::components::connected_components;
    use crate::graph::traits::OrdinalGraph;
    use crate::graph::types::UnListGraph;
    use itertools::assert_equal;

    #[test]
    fn connected_components_base_case() {
        // 0
        // | \   3   5
        // |  1  |   |
        // | /   4   6
        // 2
        let graph = UnListGraph::from_ordinal(
            vec![(); 7],
            vec![(0, 1, ()), (0, 2, ()), (2, 1, ()), (3, 4, ()), (5, 6, ())],
        );

        let expected_components =
            Vec::from([(0, 0), (1, 0), (2, 0), (3, 1), (4, 1), (5, 2), (6, 2)]);
        let components = connected_components(&graph);
        assert_equal(components, expected_components);
    }
}
