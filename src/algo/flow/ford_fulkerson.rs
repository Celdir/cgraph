use crate::algo::errors::AlgoError;
use crate::graph::flow::FlowGraph;
use crate::iter::dfs::dfs_where;
use crate::iter::traits::Traversal;

pub fn ford_fulkerson<'a, G>(
    graph: &'a mut G,
    source: G::NId,
    sink: G::NId,
) -> Result<G::FlowVal, AlgoError>
where
    G: FlowGraph,
{
    // TODO: maybe just change this to "node not found" error with custom error message
    if !graph.contains_node(source) {
        return Err(AlgoError::SourceNotFound);
    }
    if !graph.contains_node(sink) {
        return Err(AlgoError::SinkNotFound);
    }

    let mut flow = G::FlowVal::default();
    while let Some(path) = dfs_where(graph, source, |edge, _| edge.has_residual()).path_to(sink) {
        let path_flow = path.edges().map(|edge| edge.residual()).min().unwrap();
        flow = flow + path_flow;

        let edge_ids: Vec<_> = path.edges().map(|edge| edge.id()).collect();
        for id in edge_ids {
            graph
                .increase_flow(id, path_flow)
                .expect("Residual flow should be sufficient");
        }
    }

    Ok(flow)
}

#[cfg(test)]
mod tests {
    use crate::algo::errors::AlgoError;
    use crate::algo::flow::ford_fulkerson::ford_fulkerson;
    use crate::graph::flow::FlowGraph;
    use crate::graph::traits::OrdinalGraph;
    use crate::graph::types::FlatFlowGraph;

    #[test]
    fn ford_fulkerson_base_case() {
        let mut graph = FlatFlowGraph::new();
        for _ in 0..6 {
            graph.insert_node(());
        }
        graph
            .insert_flow_edge(0, 1, 10)
            .expect("node ids should exist");
        graph
            .insert_flow_edge(0, 3, 8)
            .expect("node ids should exist");
        graph
            .insert_flow_edge(1, 3, 2)
            .expect("node ids should exist");
        graph
            .insert_flow_edge(1, 2, 5)
            .expect("node ids should exist");
        graph
            .insert_flow_edge(3, 4, 10)
            .expect("node ids should exist");
        graph
            .insert_flow_edge(4, 2, 8)
            .expect("node ids should exist");
        graph
            .insert_flow_edge(2, 5, 7)
            .expect("node ids should exist");
        graph
            .insert_flow_edge(4, 5, 10)
            .expect("node ids should exist");

        let flow = ford_fulkerson(&mut graph, 0, 5).expect("flow should exist");
        assert_eq!(flow, 15);
    }

    #[test]
    fn ford_fulkerson_source_not_found() {
        let mut graph = FlatFlowGraph::<(), i32>::new();
        let flow_err = ford_fulkerson(&mut graph, 0, 5).unwrap_err();
        assert_eq!(flow_err, AlgoError::SourceNotFound);
    }

    #[test]
    fn ford_fulkerson_sink_not_found() {
        let mut graph = FlatFlowGraph::<(), i32>::new();
        graph.insert_node(());
        let flow_err = ford_fulkerson(&mut graph, 0, 5).unwrap_err();
        assert_eq!(flow_err, AlgoError::SinkNotFound);
    }
}
