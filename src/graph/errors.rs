use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphError {
    #[error("node {0:?} not found in graph")]
    NodeNotFound(String),

    #[error("edge {0:?} not found in graph")]
    EdgeNotFound(String),

    #[error("flow error: {0}")]
    FlowError(#[from] FlowError),

    #[error("unknown graph error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("back edge not found for edge {0:?} in flow graph")]
    BackEdgeNotFound(String),

    #[error("insufficient remaining capacity in edge {0:?} to increase flow: {1:?}")]
    InsufficientCapacity(String, String),
}
