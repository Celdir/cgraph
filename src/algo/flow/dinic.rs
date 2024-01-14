use crate::algo::errors::AlgoError;
use crate::graph::flow::FlowGraph;
use crate::graph::traits::GraphIter;
use crate::iter::bfs::bfs_where;

use ahash::AHashMap;
use std::cmp::min;
use std::iter::Peekable;
use std::vec::IntoIter;

pub fn dinic<'a, G>(graph: &'a mut G, source: G::NId, sink: G::NId) -> Result<G::FlowVal, AlgoError>
where
    G: FlowGraph + GraphIter,
{
    // TODO: maybe just change this to "node not found" error with custom error message
    if !graph.contains_node(source) {
        return Err(AlgoError::SourceNotFound(format!("{:?}", source)));
    }
    if !graph.contains_node(sink) {
        return Err(AlgoError::SinkNotFound(format!("{:?}", sink)));
    }

    let mut flow = G::FlowVal::default();
    while let Some(bf) = blocking_flow(graph, source, sink) {
        flow = flow + bf;
    }

    Ok(flow)
}

fn blocking_flow<'a, G>(graph: &'a mut G, source: G::NId, sink: G::NId) -> Option<G::FlowVal>
where
    G: FlowGraph + GraphIter,
{
    let levels = levels(graph, source, sink)?;

    let mut flow = G::FlowVal::default();
    let adj = adj_ids(graph);
    let mut blocking_flow = BlockingFlow {
        graph,
        adj,
        levels,
        sink,
    };
    while let Some(path_flow) = blocking_flow.dfs(source, None) {
        flow = flow + path_flow;
    }

    Some(flow)
}

struct BlockingFlow<'a, G, I>
where
    G: FlowGraph,
    I: Iterator<Item = (G::EId, G::NId)>,
{
    graph: &'a mut G,
    adj: AHashMap<G::NId, Peekable<I>>,
    levels: AHashMap<G::NId, usize>,
    sink: G::NId,
}

impl<'a, G, I> BlockingFlow<'a, G, I>
where
    G: FlowGraph,
    I: Iterator<Item = (G::EId, G::NId)>,
{
    fn dfs(&mut self, cur: G::NId, pushed: Option<G::FlowVal>) -> Option<G::FlowVal> {
        if cur == self.sink {
            return pushed;
        }
        while let Some(&(edge_id, node_id)) = self.adj.get_mut(&cur).unwrap().peek() {
            let edge = self.graph.edge(edge_id).unwrap();
            if self.levels[&node_id] == self.levels[&cur] + 1 && edge.has_residual() {
                let new_pushed = Some(pushed.map_or(edge.residual(), |f| min(f, edge.residual())));
                if let Some(flow) = self.dfs(node_id, new_pushed) {
                    self.graph
                        .increase_flow(edge_id, flow)
                        .expect("Residual flow should be sufficient");
                    return Some(flow);
                }
            }
            self.adj.get_mut(&cur).unwrap().next();
        }

        None
    }
}

// calculate levels (distance from source) for nodes in residual graph
fn levels<'a, G>(graph: &'a mut G, source: G::NId, sink: G::NId) -> Option<AHashMap<G::NId, usize>>
where
    G: FlowGraph,
{
    let mut levels = AHashMap::new();

    for (edge_opt, node) in bfs_where(graph, source, |edge, _| edge.has_residual()) {
        match edge_opt {
            Some(edge) => {
                levels.insert(node.id(), levels[&edge.other(node.id())] + 1);
            }
            _ => {
                levels.insert(node.id(), 0);
            }
        }
    }

    if !levels.contains_key(&sink) {
        return None;
    }

    Some(levels)
}

// adj_ids iterators without graph's lifetime bound
fn adj_ids<'a, G>(graph: &'a G) -> AHashMap<G::NId, Peekable<IntoIter<(G::EId, G::NId)>>>
where
    G: FlowGraph + GraphIter,
{
    let mut adj = AHashMap::new();
    for node in graph.nodes() {
        adj.insert(
            node.id(),
            graph
                .adj_ids(node.id())
                .unwrap()
                .collect::<Vec<_>>()
                .into_iter()
                .peekable(),
        );
    }

    adj
}

#[cfg(test)]
mod tests {
    use crate::algo::errors::AlgoError;
    use crate::algo::flow::dinic::dinic;
    use crate::graph::flow::FlowGraph;
    use crate::graph::traits::OrdinalGraph;
    use crate::graph::types::FlowFlatGraph;
    use std::matches;

    #[test]
    fn dinic_base_case() {
        let mut graph = FlowFlatGraph::new();
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

        let flow = dinic(&mut graph, 0, 5).expect("flow should exist");
        assert_eq!(flow, 15);
    }

    #[test]
    fn dinic_source_not_found() {
        let mut graph = FlowFlatGraph::<(), i32>::new();
        let flow_err = dinic(&mut graph, 0, 5).unwrap_err();
        assert!(matches!(flow_err, AlgoError::SourceNotFound(..)));
    }

    #[test]
    fn dinic_sink_not_found() {
        let mut graph = FlowFlatGraph::<(), i32>::new();
        graph.insert_node(());
        let flow_err = dinic(&mut graph, 0, 5).unwrap_err();
        assert!(matches!(flow_err, AlgoError::SinkNotFound(..)));
    }
}
