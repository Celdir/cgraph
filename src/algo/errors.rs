use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AlgoError {
    #[error("Graph contains cycles and thus has no topological ordering")]
    NoTopologicalOrdering,

    #[error("Specified start node does not exist in the graph")]
    StartNodeNotFound,

    #[error("No path from start node to end node")]
    NoPathFromStartToEnd,

    #[error("Specified source node does not exist in the flow graph")]
    SourceNotFound,

    #[error("Specified sink node does not exist in the flow graph")]
    SinkNotFound,

    #[error("unimplemented")]
    Unimplemented,
}
