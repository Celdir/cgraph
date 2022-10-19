/*use crate::graph::structure::edge::Edge;
use crate::graph::structure::graph::{Graph, KeyedGraph, UndirectedGraph};
use crate::graph::structure::node::Node;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::convert::From;
use std::fmt::Display;
use std::hash::Hash;

// TODO: implement pooling for re-using deleted node and edge ids
// alternative, just use hashmaps to store nodes and edges
//

#[derive(Clone)]
struct InternalEdge<Id, E> {
    u: Id,
    v: Id,
    e: E,
}

#[derive(Clone)]
pub struct MapGraph<Id, N, E> {
    nodes: HashMap<Id, N>,
    edges: HashMap<usize, InternalEdge<Id, E>>,
    next_edge_id: usize,
    adj: HashMap<Id, HashMap<Id, usize>>,
}

impl<Id: Eq + Hash + Copy, N, E> MapGraph<Id, N, E> {
    pub fn new() -> MapGraph<Id, N, E> {
        MapGraph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            next_edge_id: 0,
            adj: HashMap::new(),
        }
    }
    pub fn with_capacity(num_nodes: usize, num_edges: usize) -> MapGraph<Id, N, E> {
        MapGraph {
            nodes: HashMap::with_capacity(num_nodes),
            edges: HashMap::with_capacity(num_edges),
            next_edge_id: 0,
            adj: HashMap::with_capacity(num_nodes),
        }
    }
}

impl<'a, Id: 'a + Eq + Hash + Copy, N: 'a, E: 'a> Graph<'a> for MapGraph<Id, N, E> {
    type N = N;
    type NId = Id;
    type E = E;
    type EId = usize;

    type NodeIterator = NodeIterator<'a, Id, N>;
    type EdgeIterator = EdgeIterator<'a, Id, E>;
    type AdjIterator = AdjIterator<'a, Id, N, E>;

    fn len(&self) -> (usize, usize) {
        (self.nodes.len(), self.edges.len())
    }

    fn contains_node(&self, id: Id) -> bool {
        self.nodes.contains_key(&id)
    }

    fn node(&self, id: Id) -> Option<Node<'a, Id, N>> {
        self.nodes.get(&id).map(|n| Node::new(id, n))
    }

    fn node_data(&self, id: Id) -> Option<&'a N> {
        self.nodes.get(&id)
    }

    fn node_data_mut(&mut self, id: Id) -> Option<&'a mut N> {
        self.nodes.get_mut(&id)
    }

    fn degree(&self, u: Id) -> usize {
        self.adj.get(&u).map_or(0, |adj_map| adj_map.len())
    }

    fn remove_node(&mut self, id: Id) -> Option<N> {
        self.clear_node(id);
        self.nodes.remove(&id)
    }

    fn clear_node(&mut self, id: Id) -> Option<()> {
        let edge_ids: Vec<usize> = self.adj(id)?.map(|(edge, _)| edge.id()).collect();
        for id in edge_ids {
            self.remove_edge(id);
        }
        Some(())
    }

    fn contains_edge(&self, u: Id, v: Id) -> bool {
        self.adj.get(&u).is_some() && self.adj[&u].contains_key(&v)
    }

    fn edge(&self, id: usize) -> Option<Edge<'a, Id, usize, E>> {
        let edge = self.edges.get(&id)?;
        Some(Edge::new(id, edge.u, edge.v, &edge.e))
    }

    fn between(&self, u: Id, v: Id) -> Option<Edge<'a, Id, usize, E>> {
        let &edge_id = self.adj.get(&u)?.get(&v)?;
        let edge = self.edges.get(&edge_id)?;
        Some(Edge::new(edge_id, u, v, &edge.e))
    }

    fn edge_data(&self, id: usize) -> Option<&'a E> {
        Some(&self.edges.get(&id).as_ref()?.e)
    }

    fn edge_data_mut(&mut self, id: usize) -> Option<&'a mut E> {
        Some(&mut self.edges.get_mut(&id)?.e)
    }

    fn insert_edge(&mut self, u: Id, v: Id, edge: E) -> Option<usize> {
        if !self.contains_node(u) || !self.contains_node(v) {
            return None;
        }

        let id = self.next_edge_id;
        self.next_edge_id += 1;

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
        self.adj
            .get_mut(&internal_edge.v)
            .unwrap()
            .remove(&internal_edge.u);
        Some(internal_edge.e)
    }

    fn nodes(&self) -> NodeIterator<'a, Id, N> {
        NodeIterator {
            inner: self.nodes.iter(),
        }
    }

    fn edges(&self) -> EdgeIterator<'a, Id, E> {
        EdgeIterator {
            inner: self.edges.iter(),
        }
    }

    fn adj(&self, u: Id) -> Option<AdjIterator<'a, Id, N, E>> {
        Some(AdjIterator {
            u,
            nodes: &self.nodes,
            edges: &self.edges,
            iter: self.adj.get(&u)?.iter(),
        })
    }
}

impl<'a, Id: 'a + Eq + Hash + Copy, N: 'a, E: 'a> UndirectedGraph<'a> for MapGraph<Id, N, E> {}

impl<'a, Id: 'a + Eq + Hash + Copy, N: 'a, E: 'a> KeyedGraph<'a> for MapGraph<Id, N, E> {
    // returns previous node data if there was any, deletes any existing edges at that id
    // to change node data without removing edges, use node_data_mut()
    fn put_node(&mut self, id: Id, node: N) -> Option<N> {
        let previous = self.remove_node(id);
        self.nodes.insert(id, node);
        self.adj.insert(id, HashMap::new());

        previous
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

pub struct NodeIterator<'a, Id, N> {
    inner: Iter<'a, Id, N>,
}

impl<'a, Id: Copy + Eq + Hash, N: 'a> Iterator for NodeIterator<'a, Id, N> {
    type Item = Node<'a, Id, N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, n)| Node::new(*id, n))
    }
}

pub struct EdgeIterator<'a, Id, E> {
    inner: Iter<'a, usize, InternalEdge<Id, E>>,
}

impl<'a, Id: Copy + Eq + Hash, E: 'a> Iterator for EdgeIterator<'a, Id, E> {
    type Item = Edge<'a, Id, usize, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(&id, edge)| Edge::new(id, edge.u, edge.v, &edge.e))
    }
}

pub struct AdjIterator<'a, Id, N, E> {
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

#[cfg(test)]
mod tests {
    use crate::graph::structure::graph::{Graph, KeyedGraph};
    use crate::graph::structure::mapgraph::MapGraph;

    #[test]
    fn puts_and_removes() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let mut graph = MapGraph::new();
        graph.put_node("A", ());
        graph.put_node("B", ());
        graph.put_node("C", ());
        graph.put_node("D", ());
        graph.insert_edge("A", "B", 5).expect("nodes should exist");
        graph.insert_edge("A", "C", 2).expect("nodes should exist");
        graph.insert_edge("C", "D", 1).expect("nodes should exist");
        graph.insert_edge("B", "D", 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        graph.remove_node("A").expect("node should exist");

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);
    }
}*/
