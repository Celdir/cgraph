use crate::graph::structure::{Graph};
use std::convert::From;
use std::cmp::max;

pub struct SimpleGraph<N, E> {
    nodes: Vec<N>,
    adjacency: Vec<Vec<(usize, E)>>,
    edges_len: usize,
}

impl<'a, N: 'a, E: 'a> Graph<'a> for SimpleGraph<N, E> {
    type Node = N;
    type Edge = E;
    type Idx = usize;

    fn nodes(&self) -> Box<dyn Iterator<Item = (usize, &N)> + '_> {
        Box::new(self.nodes.iter().enumerate())
    }
    fn edges(&self) -> Box<dyn Iterator<Item = (usize, usize, &E)> + '_> {
        Box::new(self.adjacency.iter().enumerate().flat_map(|(u, u_adj)| {
            u_adj
                .iter()
                .map(move |(v, edge)| (u.clone(), v.clone(), edge))
        }))
    }

    fn adj(
        &self,
        u: usize,
    ) -> Box<dyn Iterator<Item = (usize, usize, &E)> + '_> {
        Box::new(
            self.adjacency[u]
                .iter()
                .map(move |(v, edge)| (u.clone(), v.clone(), edge)),
        )
    }

    fn nodes_len(&self) -> usize {
        self.nodes.len()
    }
    fn edges_len(&self) -> usize {
        self.edges_len
    }

    fn get_node(&self, u: usize) -> Option<&N> {
        self.nodes.get(u)
    }
    fn get_edge(&self, u: usize, v: usize) -> Option<&E> {
        let (_, edge) = self.adjacency.get(u)?.iter().find(|&&(x, _)| x == v)?;
        Some(&edge)
    }

    fn insert_node(&mut self, node: N) -> usize {
        self.nodes.push(node);
        self.adjacency.push(Vec::new());
        self.nodes.len() - 1
    }
    fn insert_edge(&mut self, u: usize, v: usize, edge: E) {
        self.adjacency[u].push((v, edge));
    }

    //fn remove_node(&mut self, node: N);
    //fn remove_edge(&mut self, edge: E);
}

impl From<Vec<(usize, usize)>> for SimpleGraph<(), ()> {
    fn from(edges: Vec<(usize, usize)>) -> Self {
        let max_idx = edges.iter().map(|(u, v)| max(u, v)).max();
        let size = match max_idx {
            Some(idx) => idx+1,
            None => 0
        };
        let mut adjacency = vec![Vec::new(); size];
        for (u, v) in &edges {
            adjacency[*u].push((*v, ()));
        }

        SimpleGraph {
            nodes: vec![(); size],
            adjacency,
            edges_len: edges.len(),
        }
    }
}
