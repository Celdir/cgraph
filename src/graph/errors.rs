use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("node not found in graph")]
    NodeNotFound,

    #[error("edge not found in graph")]
    EdgeNotFound,

    #[error("unknown graph error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum FlowGraphError {
    #[error("forward edge not found in flow graph")]
    ForwardEdgeNotFound,

    #[error("back edge not found in flow graph")]
    BackEdgeNotFound,

    #[error("insufficient remaining capacity to increase flow")]
    InsufficientCapacity,
}
