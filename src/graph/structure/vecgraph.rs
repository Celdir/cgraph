use crate::graph::structure::edge::Edge;
use crate::graph::structure::node::Node;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::convert::From;
use std::iter;
use std::ops::Index;

// TODO: implement pooling for re-using deleted node and edge ids
// alternative, just use hashmaps to store nodes and edges
//

struct InternalEdge<E> {
    u: usize,
    v: usize,
    e: E,
}

// Directed graph
pub struct VecGraph<N, E> {
    nodes: Vec<Option<N>>,
    nodes_len: usize,
    edges: Vec<Option<InternalEdge<E>>>,
    edges_len: usize,
    out_adj: Vec<HashMap<usize, usize>>,
    in_adj: Vec<HashMap<usize, usize>>,
}

impl<N, E> VecGraph<N, E> {
    fn new() -> VecGraph<N, E> {
        VecGraph {
            nodes: Vec::new(),
            nodes_len: 0,
            edges: Vec::new(),
            edges_len: 0,
            out_adj: Vec::new(),
            in_adj: Vec::new(),
        }
    }

    fn with_capacity(num_nodes: usize, num_edges: usize) -> VecGraph<N, E> {
        VecGraph {
            nodes: Vec::with_capacity(num_nodes),
            nodes_len: 0,
            edges: Vec::with_capacity(num_edges),
            edges_len: 0,
            out_adj: Vec::with_capacity(num_nodes),
            in_adj: Vec::with_capacity(num_nodes),
        }
    }

    fn len(&self) -> (usize, usize) {
        (self.nodes_len, self.edges_len)
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

    fn insert_node(&mut self, node: N) -> usize {
        let id = self.nodes.len();
        self.nodes.push(Some(node));
        self.nodes_len += 1;
        self.out_adj.push(HashMap::new());
        self.in_adj.push(HashMap::new());
        id
    }

    fn remove_node(&mut self, id: usize) -> Option<N> {
        self.clear_node(id);
        let node = self.nodes.remove(id)?;
        self.nodes_len -= 1;
        Some(node)
    }

    fn contains_node(&self, id: usize) -> bool {
        self.nodes.get(id).is_some() && self.nodes[id].is_some()
    }

    fn nodes(&self) -> impl Iterator<Item = Node<usize, N>> {
        self.nodes
            .iter()
            .enumerate()
            .filter(|(_, opt)| opt.is_some())
            .map(|(id, opt)| Node::new(id, opt.as_ref().unwrap()))
    }

    fn edge(&self, id: usize) -> Option<Edge<usize, usize, E>> {
        match self.edges.get(id) {
            Some(Some(edge)) => Some(Edge::new(id, edge.u, edge.v, &edge.e)),
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

    fn edges(&self) -> impl Iterator<Item = Edge<usize, usize, E>> {
        self.edges
            .iter()
            .enumerate()
            .filter(|(_, opt)| opt.is_some())
            .map(|(id, opt)| (id, opt.as_ref().unwrap()))
            .map(|(id, edge)| Edge::new(id, edge.u, edge.v, &edge.e))
    }

    fn contains_edge(&self, u: usize, v: usize) -> bool {
        self.out_adj.get(u).is_some() && self.out_adj[u].contains_key(&v)
    }

    fn between(&self, u: usize, v: usize) -> Option<Edge<usize, usize, E>> {
        let &edge_id = self.out_adj.get(u)?.get(&v)?;
        match self.edges.get(edge_id) {
            Some(Some(edge)) => Some(Edge::new(edge_id, edge.u, edge.v, &edge.e)),
            _ => None,
        }
    }

    fn between_multi(
        &self,
        u: usize,
        v: usize,
    ) -> Option<impl Iterator<Item = Edge<usize, usize, E>>> {
        let &edge_id = self.out_adj.get(u)?.get(&v)?;
        match self.edges.get(edge_id) {
            Some(Some(edge)) => Some(iter::once(Edge::new(edge_id, edge.u, edge.v, &edge.e))),
            _ => None,
        }
    }

    // Returns out edges for directed graph or all edges for undirected
    fn adj(&self, u: usize) -> Option<AdjIterator<N, E>> {
        self.out_edges(u)
    }

    fn degree(&self, u: usize) -> usize {
        self.in_degree(u) + self.out_degree(u)
    }

    fn out_edges(&self, u: usize) -> Option<AdjIterator<N, E>> {
        Some(AdjIterator {
            u,
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
            u,
            nodes: &self.nodes,
            edges: &self.edges,
            iter: self.in_adj.get(u)?.iter(),
        })
    }

    fn in_degree(&self, u: usize) -> usize {
        self.in_adj.get(u).map_or(0, |adj_map| adj_map.len())
    }
}

impl<N, E> From<(Vec<N>, Vec<(usize, usize, E)>)> for VecGraph<N, E> {
    fn from(data: (Vec<N>, Vec<(usize, usize, E)>)) -> Self {
        let (nodes, edges) = data;
        let mut graph = VecGraph::with_capacity(nodes.len(), edges.len());

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

struct AdjIterator<'a, N, E> {
    u: usize,
    nodes: &'a Vec<Option<N>>,
    edges: &'a Vec<Option<InternalEdge<E>>>,
    iter: Iter<'a, usize, usize>,
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
