use crate::algo::errors::AlgoError;
use crate::graph::flow::FlowGraph;

pub fn dinic<'a, G>(graph: &'a mut G) -> Result<G::FlowVal, AlgoError>
where
    G: FlowGraph,
{
    return Err(AlgoError::Unimplemented);
}
