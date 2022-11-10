use crate::graph::cgraph::CGraph;
use crate::graph::containers::adj::adj_map::AdjMap;
use crate::graph::containers::adj::di::Di;
use crate::graph::containers::adj::un::Un;
use crate::graph::containers::edge::edge_stable_vec::EdgeStableVec;
use crate::graph::containers::node::node_map::NodeMap;

pub type DiGraph<NC, EC, AC> = CGraph<NC, EC, Di<AC>>;
pub type UnGraph<NC, EC, AC> = CGraph<NC, EC, Un<AC>>;

pub type DiMapGraph<Id, N, E> = CGraph<NodeMap<Id, N>, EdgeStableVec<Id, E>, Di<AdjMap<Id, usize>>>;
pub type UnMapGraph<Id, N, E> = CGraph<NodeMap<Id, N>, EdgeStableVec<Id, E>, Un<AdjMap<Id, usize>>>;
