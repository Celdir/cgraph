use crate::graph::structure::edge::Edge;
use crate::graph::structure::node::Node;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::convert::From;
use std::fmt::Display;
use std::hash::Hash;
use std::iter;
use std::ops::Index;

// TODO: implement pooling for re-using deleted node and edge ids
// alternative, just use hashmaps to store nodes and edges
//

struct InternalEdge<Id, E> {
    u: Id,
    v: Id,
    e: E,
}

pub struct MapGraph<Id, N, E> {
    nodes: HashMap<Id, N>,
    edges: HashMap<usize, InternalEdge<Id, E>>,
    adj: HashMap<Id, HashMap<Id, usize>>,
}

impl<Id: Eq + Hash + Copy, N, E> MapGraph<Id, N, E> {
    fn new() -> MapGraph<Id, N, E> {
        MapGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            adj: HashMap::new(),
        }
    }
    fn with_capacity(num_nodes: usize, num_edges: usize) -> MapGraph<Id, N, E> {
        MapGraph {
            nodes: HashMap::with_capacity(num_nodes),
            edges: HashMap::with_capacity(num_edges),
            adj: HashMap::with_capacity(num_nodes),
        }
    }

    fn len(&self) -> (usize, usize) {
        (self.nodes.len(), self.edges.len())
    }

    fn node(&self, id: Id) -> Option<Node<Id, N>> {
        self.nodes.get(&id).map(|n| Node::new(id, n))
    }

    fn node_data(&self, id: Id) -> Option<&N> {
        self.nodes.get(&id)
    }

    fn node_data_mut(&mut self, id: Id) -> Option<&mut N> {
        self.nodes.get_mut(&id)
    }

    fn insert_node(&mut self, node: N) -> Id {
        panic!("MapGraph nodes can only be inserted with put_node");
    }

    // returns previous node data if there was any, deletes any existing edges at that id
    // to change node data without removing edges, use node_data_mut()
    fn put_node(&mut self, id: Id, node: N) -> Option<N> {
        let previous = self.remove_node(id);
        self.nodes.insert(id, node);
        self.adj.insert(id, HashMap::new());

        previous
    }

    fn remove_node(&mut self, id: Id) -> Option<N> {
        // self.clear_node(id);
        self.nodes.remove(&id)
    }

    fn contains_node(&self, id: Id) -> bool {
        self.nodes.contains_key(&id)
    }

    fn nodes(&self) -> impl Iterator<Item = Node<Id, N>> {
        self.nodes.iter().map(|(id, n)| Node::new(*id, n))
    }

    fn edge(&self, id: usize) -> Option<Edge<Id, usize, E>> {
        let edge = self.edges.get(&id)?;
        Some(Edge::new(id, edge.u, edge.v, &edge.e))
    }

    fn edge_data(&self, id: usize) -> Option<&E> {
        Some(&self.edges.get(&id).as_ref()?.e)
    }

    fn edge_data_mut(&mut self, id: usize) -> Option<&mut E> {
        Some(&mut self.edges.get_mut(&id)?.e)
    }

    fn insert_edge(&mut self, u: Id, v: Id, edge: E) -> Option<usize> {
        if !self.contains_node(u) || !self.contains_node(v) {
            return None;
        }

        let id = self.edges.len();
        self.edges.insert(id, InternalEdge { u, v, e: edge });
        self.adj.get_mut(&u).unwrap().insert(v, id);
        self.adj.get_mut(&v).unwrap().insert(u, id);
        Some(id)
    }

    fn remove_edge(&mut self, id: usize) -> Option<E> {
        let internal_edge = self.edges.remove(&id)?;
        self.adj
            .get_mut(&internal_edge.u)
            .unwrap()
            .remove(&internal_edge.v);
        Some(internal_edge.e)
    }

    fn clear_node(&mut self, id: Id) -> Option<()> {
        let edge_ids: Vec<usize> = self.adj(id)?.map(|(edge, _)| edge.id()).collect();
        for id in edge_ids {
            self.remove_edge(id);
        }
        Some(())
    }

    fn edges(&self) -> impl Iterator<Item = Edge<Id, usize, E>> {
        self.edges
            .iter()
            .map(|(&id, edge)| Edge::new(id, edge.u, edge.v, &edge.e))
    }

    fn contains_edge(&self, u: Id, v: Id) -> bool {
        self.adj.get(&u).is_some() && self.adj[&u].contains_key(&v)
    }

    fn between(&self, u: Id, v: Id) -> Option<Edge<Id, usize, E>> {
        let &edge_id = self.adj.get(&u)?.get(&v)?;
        let edge = self.edges.get(&edge_id)?;
        Some(Edge::new(edge_id, u, v, &edge.e))
    }

    fn adj(&self, u: Id) -> Option<AdjIterator<Id, N, E>> {
        Some(AdjIterator {
            u,
            nodes: &self.nodes,
            edges: &self.edges,
            iter: self.adj.get(&u)?.iter(),
        })
    }

    fn degree(&self, u: Id) -> usize {
        self.adj.get(&u).map_or(0, |adj_map| adj_map.len())
    }
}

impl<Id: Eq + Hash + Copy + Display, N, E> From<(Vec<(Id, N)>, Vec<(Id, Id, E)>)>
    for MapGraph<Id, N, E>
{
    fn from(data: (Vec<(Id, N)>, Vec<(Id, Id, E)>)) -> Self {
        let (nodes, edges) = data;
        let mut graph = MapGraph::with_capacity(nodes.len(), edges.len());

        for (id, n) in nodes {
            graph.put_node(id, n);
        }

        for (u, v, e) in edges {
            graph
                .insert_edge(u, v, e)
                .expect(format!("{} -> {} is not a valid edge: no such node ids.", u, v).as_str());
        }

        graph
    }
}

struct AdjIterator<'a, Id, N, E> {
    u: Id,
    nodes: &'a HashMap<Id, N>,
    edges: &'a HashMap<usize, InternalEdge<Id, E>>,
    iter: Iter<'a, Id, usize>,
}

impl<'a, Id: Eq + Hash + Copy, N, E> Iterator for AdjIterator<'a, Id, N, E> {
    type Item = (Edge<'a, Id, usize, E>, Node<'a, Id, N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(&v, &id)| {
            let internal_edge = &self.edges[&id];
            let edge = Edge::new(id, self.u, v, &internal_edge.e);
            let node = Node::new(v, &self.nodes[&v]);
            (edge, node)
        })
    }
}
