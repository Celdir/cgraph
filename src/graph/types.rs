use crate::graph::cgraph::CGraph;
use crate::graph::containers::adj::adj_list::AdjList;
use crate::graph::containers::adj::adj_map::AdjMap;
use crate::graph::containers::adj::di::Di;
use crate::graph::containers::adj::flat_adj_list::FlatAdjList;
use crate::graph::containers::adj::un::Un;
use crate::graph::containers::edge::edge_stable_vec::EdgeStableVec;
use crate::graph::containers::node::node_map::NodeMap;
use crate::graph::containers::node::node_stable_vec::NodeStableVec;
use crate::graph::flow::Flow;

pub type DiGraph<NC, EC, AC> = CGraph<NC, EC, Di<AC>>;
pub type UnGraph<NC, EC, AC> = CGraph<NC, EC, Un<AC>>;

pub type DiMapGraph<Id, N, E> = CGraph<NodeMap<Id, N>, EdgeStableVec<Id, E>, Di<AdjMap<Id, usize>>>;
pub type UnMapGraph<Id, N, E> = CGraph<NodeMap<Id, N>, EdgeStableVec<Id, E>, Un<AdjMap<Id, usize>>>;

pub type DiListGraph<N, E> = CGraph<NodeStableVec<N>, EdgeStableVec<usize, E>, Di<AdjList<usize>>>;
pub type UnListGraph<N, E> = CGraph<NodeStableVec<N>, EdgeStableVec<usize, E>, Un<AdjList<usize>>>;

pub type DiFlatGraph<N, E> =
    CGraph<NodeStableVec<N>, EdgeStableVec<usize, E>, Di<FlatAdjList<usize>>>;
pub type UnFlatGraph<N, E> =
    CGraph<NodeStableVec<N>, EdgeStableVec<usize, E>, Un<FlatAdjList<usize>>>;

pub type FlatFlowGraph<N, V> =
    CGraph<NodeStableVec<N>, EdgeStableVec<usize, Flow<V>>, FlatAdjList<usize>>;
