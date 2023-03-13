use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AlgoError {
    #[error("Graph contains cycles and thus has no topological ordering")]
    NoTopologicalOrdering,

    #[error("Start node {0:?} does not exist in the graph")]
    StartNodeNotFound(String),

    #[error("No path from start node {0:?} to end node {1:?}")]
    NoPathFromStartToEnd(String, String),

    #[error("Source node {0:?} does not exist in the flow graph")]
    SourceNotFound(String),

    #[error("Sink node {0:?} does not exist in the flow graph")]
    SinkNotFound(String),

    #[error("unimplemented")]
    Unimplemented,
}
