use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::{DirectedGraph, Graph, MultiGraph, OrdinalGraph};
use crate::graph::structure::node::Node;
use std::collections::hash_map;
use std::collections::HashMap;
use std::convert::From;
use std::iter;
use std::slice;

// TODO: implement pooling for re-using deleted node and edge ids
// alternative, just use hashmaps to store nodes and edges
//

#[derive(Clone)]
struct InternalEdge<E> {
    u: usize,
    v: usize,
    e: E,
}

// Directed graph
#[derive(Clone)]
pub struct StableVecGraph<N, E>
where
    N: Clone,
    E: Clone,
{
    nodes: Vec<Option<N>>,
    nodes_len: usize,
    edges: Vec<Option<InternalEdge<E>>>,
    edges_len: usize,
    out_adj: Vec<HashMap<usize, usize>>,
    in_adj: Vec<HashMap<usize, usize>>,
}

impl<N: Clone, E: Clone> StableVecGraph<N, E> {
    pub fn new() -> StableVecGraph<N, E> {
        StableVecGraph {
            nodes: Vec::new(),
            nodes_len: 0,
            edges: Vec::new(),
            edges_len: 0,
            out_adj: Vec::new(),
            in_adj: Vec::new(),
        }
    }

    pub fn with_capacity(num_nodes: usize, num_edges: usize) -> StableVecGraph<N, E> {
        StableVecGraph {
            nodes: Vec::with_capacity(num_nodes),
            nodes_len: 0,
            edges: Vec::with_capacity(num_edges),
            edges_len: 0,
            out_adj: Vec::with_capacity(num_nodes),
            in_adj: Vec::with_capacity(num_nodes),
        }
    }
}

impl<'a, N, E> Graph<'a> for StableVecGraph<N, E>
where
    N: 'a + Clone,
    E: 'a + Clone,
{
    type N = N;
    type NId = usize;
    type E = E;
    type EId = usize;

    type NodeIterator = NodeIterator<'a, N>;
    type EdgeIterator = EdgeIterator<'a, E>;
    type AdjIterator = AdjIterator<'a, N, E>;

    fn len(&self) -> (usize, usize) {
        (self.nodes_len, self.edges_len)
    }

    fn contains_node(&self, id: usize) -> bool {
        self.nodes.get(id).is_some() && self.nodes[id].is_some()
    }

    fn node(&self, id: usize) -> Option<Node<usize, N>> {
        match self.nodes.get(id) {
            Some(Some(node)) => Some(Node::new(id, node)),
            _ => None,
        }
    }

    fn node_data(&self, id: usize) -> Option<&N> {
        self.nodes.get(id)?.as_ref()
    }

    fn node_data_mut(&mut self, id: usize) -> Option<&mut N> {
        self.nodes.get_mut(id)?.as_mut()
    }

    fn degree(&self, u: usize) -> usize {
        self.in_degree(u) + self.out_degree(u)
    }

    fn remove_node(&mut self, id: usize) -> Option<N> {
        self.clear_node(id);
        let node = self.nodes.remove(id)?;
        self.nodes_len -= 1;
        Some(node)
    }

    fn clear_node(&mut self, id: usize) -> Option<()> {
        let edge_ids: Vec<usize> = self
            .out_edges(id)?
            .chain(self.in_edges(id)?)
            .map(|(edge, _)| edge.id())
            .collect();
        for id in edge_ids {
            self.remove_edge(id);
        }
        Some(())
    }

    fn contains_edge(&self, u: usize, v: usize) -> bool {
        self.out_adj.get(u).is_some() && self.out_adj[u].contains_key(&v)
    }

    fn edge(&self, id: usize) -> Option<Edge<usize, usize, E>> {
        match self.edges.get(id) {
            Some(Some(edge)) => Some(Edge::new(id, edge.u, edge.v, &edge.e)),
            _ => None,
        }
    }

    fn between(&self, u: usize, v: usize) -> Option<Edge<usize, usize, E>> {
        let &edge_id = self.out_adj.get(u)?.get(&v)?;
        match self.edges.get(edge_id) {
            Some(Some(edge)) => Some(Edge::new(edge_id, edge.u, edge.v, &edge.e)),
            _ => None,
        }
    }

    fn edge_data(&self, id: usize) -> Option<&E> {
        Some(&self.edges.get(id)?.as_ref()?.e)
    }

    fn edge_data_mut(&mut self, id: usize) -> Option<&mut E> {
        Some(&mut self.edges.get_mut(id)?.as_mut()?.e)
    }

    fn insert_edge(&mut self, u: usize, v: usize, edge: E) -> Option<usize> {
        if u >= self.nodes.len() || v >= self.nodes.len() {
            return None;
        }

        let id = self.edges.len();
        self.edges.push(Some(InternalEdge { u, v, e: edge }));
        self.edges_len += 1;
        self.out_adj[u].insert(v, id);
        self.in_adj[v].insert(u, id);
        Some(id)
    }

    fn remove_edge(&mut self, id: usize) -> Option<E> {
        let internal_edge = self.edges.remove(id)?;
        self.edges_len -= 1;
        self.out_adj[internal_edge.u].remove(&internal_edge.v);
        self.in_adj[internal_edge.v].remove(&internal_edge.u);
        Some(internal_edge.e)
    }

    fn nodes(&self) -> NodeIterator<N> {
        NodeIterator {
            inner: self.nodes.iter().enumerate(),
        }
    }

    fn edges(&self) -> EdgeIterator<E> {
        EdgeIterator {
            inner: self.edges.iter().enumerate(),
        }
    }

    // Returns out edges for directed graph or all edges for undirected
    fn adj(&self, u: usize) -> Option<AdjIterator<N, E>> {
        self.out_edges(u)
    }
}

impl<'a, N, E> OrdinalGraph<'a> for StableVecGraph<N, E>
where
    N: 'a + Clone,
    E: 'a + Clone,
{
    fn insert_node(&mut self, node: N) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Some(node));
        self.nodes_len += 1;
        self.out_adj.push(HashMap::new());
        self.in_adj.push(HashMap::new());
        id
    }
}

impl<'a, N, E> DirectedGraph<'a> for StableVecGraph<N, E>
where
    N: 'a + Clone,
    E: 'a + Clone,
{
    fn out_edges(&self, u: usize) -> Option<AdjIterator<N, E>> {
        Some(AdjIterator {
            nodes: &self.nodes,
            edges: &self.edges,
            iter: self.out_adj.get(u)?.iter(),
        })
    }

    fn out_degree(&self, u: usize) -> usize {
        self.out_adj.get(u).map_or(0, |adj_map| adj_map.len())
    }

    fn in_edges(&self, u: usize) -> Option<AdjIterator<N, E>> {
        Some(AdjIterator {
            nodes: &self.nodes,
            edges: &self.edges,
            iter: self.in_adj.get(u)?.iter(),
        })
    }

    fn in_degree(&self, u: usize) -> usize {
        self.in_adj.get(u).map_or(0, |adj_map| adj_map.len())
    }

    fn reverse(&self) -> StableVecGraph<N, E> {
        let copy: StableVecGraph<N, E> = self.clone(); let (nodes, edges): (Vec<N>, Vec<(usize, usize, E)>) = copy.into();
        let mut reverse_graph = StableVecGraph::with_capacity(nodes.len(), edges.len());
        for n in nodes {
            reverse_graph.insert_node(n);
        }
        for (u, v, e) in edges {
            reverse_graph.insert_edge(v, u, e);
        }

        reverse_graph
    }

    fn reverse_edge(&mut self, id: usize) -> Option<()> {
        let edge = self.edges.get_mut(id)?.as_mut()?;
        std::mem::swap(&mut edge.u, &mut edge.v);
        Some(())
    }
}

impl<'a, N, E> MultiGraph<'a> for StableVecGraph<N, E>
where
    N: 'a + Clone,
    E: 'a + Clone,
{
    type MultiEdgeIterator = iter::Once<Edge<'a, usize, usize, E>>;

    fn between_multi(&'a self, u: usize, v: usize) -> Option<Self::MultiEdgeIterator> {
        let &edge_id = self.out_adj.get(u)?.get(&v)?;
        match self.edges.get(edge_id) {
            Some(Some(edge)) => Some(iter::once(Edge::new(edge_id, edge.u, edge.v, &edge.e))),
            _ => None,
        }
    }
}

impl<N: Clone, E: Clone> From<(Vec<N>, Vec<(usize, usize, E)>)> for StableVecGraph<N, E> {
    fn from(data: (Vec<N>, Vec<(usize, usize, E)>)) -> Self {
        let (nodes, edges) = data;
        let mut graph = StableVecGraph::with_capacity(nodes.len(), edges.len());

        for n in nodes {
            graph.insert_node(n);
        }

        for (u, v, e) in edges {
            graph
                .insert_edge(u, v, e)
                .expect(format!("{} -> {} is not a valid edge: no such node ids.", u, v).as_str());
        }

        graph
    }
}

impl<N: Clone, E: Clone> From<StableVecGraph<N, E>> for (Vec<N>, Vec<(usize, usize, E)>) {
    fn from(graph: StableVecGraph<N, E>) -> Self {
        let nodes: Vec<N> = graph
            .nodes
            .into_iter()
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        let edges: Vec<(usize, usize, E)> = graph
            .edges
            .into_iter()
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .map(|edge| (edge.u, edge.v, edge.e))
            .collect();

        (nodes, edges)
    }
}

pub struct AdjIterator<'a, N, E> {
    nodes: &'a Vec<Option<N>>,
    edges: &'a Vec<Option<InternalEdge<E>>>,
    iter: hash_map::Iter<'a, usize, usize>,
}

impl<'a, N, E> Iterator for AdjIterator<'a, N, E> {
    type Item = (Edge<'a, usize, usize, E>, Node<'a, usize, N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&v, &id)| {
            let internal_edge = self.edges[id].as_ref().unwrap();
            let edge = Edge::new(id, internal_edge.u, internal_edge.v, &internal_edge.e);
            let node = Node::new(v, self.nodes[v].as_ref().unwrap());
            (edge, node)
        })
    }
}

pub struct NodeIterator<'a, N> {
    inner: iter::Enumerate<slice::Iter<'a, Option<N>>>,
}

impl<'a, N> Iterator for NodeIterator<'a, N> {
    type Item = Node<'a, usize, N>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let node = opt.as_ref().unwrap();
                return Some(Node::new(id, node));
            }
        }
    }
}

pub struct EdgeIterator<'a, E> {
    inner: iter::Enumerate<slice::Iter<'a, Option<InternalEdge<E>>>>,
}

impl<'a, E> Iterator for EdgeIterator<'a, E> {
    type Item = Edge<'a, usize, usize, E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (id, opt) = self.inner.next()?;
            if opt.is_some() {
                let edge = opt.as_ref().unwrap();
                return Some(Edge::new(id, edge.u, edge.v, &edge.e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::structure::graph::Graph;
    use crate::graph::structure::vecgraph::StableVecGraph;

    #[test]
    fn puts_and_removes() {
        // A --5-> B
        // |       |
        // 2       1
        // v       v
        // C --1-> D
        let mut graph = StableVecGraph::new();
        graph.insert_node(()); // 0
        graph.insert_node(()); // 1
        graph.insert_node(()); // 2
        graph.insert_node(()); // 3
        graph.insert_edge(0, 1, 5).expect("nodes should exist");
        graph.insert_edge(0, 2, 2).expect("nodes should exist");
        graph.insert_edge(2, 3, 1).expect("nodes should exist");
        graph.insert_edge(1, 3, 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        graph.remove_node(0).expect("node should exist");

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);
    }
}
