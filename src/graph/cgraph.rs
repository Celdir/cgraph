use crate::graph::containers::adj::traits::{
    AdjContainer, DirectedAdjContainer, KeyedAdjContainer, RawAdjContainer, UndirectedAdjContainer,
};
use crate::graph::containers::edge::traits::EdgeContainer;
use crate::graph::containers::node::traits::{
    KeyedNodeContainer, NodeContainer, OrdinalNodeContainer,
};
use crate::graph::edge::{Edge, EdgeMut};
use crate::graph::errors::GraphError;
use crate::graph::node::{Node, NodeMut};
use crate::graph::traits::{
    DirectedGraph, Graph, KeyedGraph, OrdinalGraph, RawGraph, UndirectedGraph, WithCapacity,
};
use std::default::Default;

pub struct CGraph<NC, EC, AC> {
    nodes: NC,
    edges: EC,
    adj: AC,
}

impl<NC, EC, AC> Graph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    type N = NC::N;
    type NId = NC::NId;
    type E = EC::E;
    type EId = EC::EId;

    type NodeIterator<'a> = NC::NodeIterator<'a> where Self: 'a;
    type NodeMutIterator<'a> = NC::NodeMutIterator<'a> where Self: 'a;

    type EdgeIterator<'a> = EC::EdgeIterator<'a> where Self: 'a;
    type EdgeMutIterator<'a> = EC::EdgeMutIterator<'a> where Self: 'a;

    type AdjIterator<'a> = GAdjIterator<'a, NC, EC, AC> where Self: 'a;
    type AdjMutIterator<'a> = GAdjMutIterator<'a, NC, EC, AC> where Self: 'a;

    fn len(&self) -> (usize, usize) {
        (self.nodes.len(), self.edges.len())
    }

    fn contains_node(&self, id: Self::NId) -> bool {
        self.nodes.contains_node(id)
    }

    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>> {
        self.nodes.node(id)
    }

    fn node_mut(&mut self, id: Self::NId) -> Option<NodeMut<Self::NId, Self::N>> {
        self.nodes.node_mut(id)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.adj.degree(u)
    }

    fn remove_node(&mut self, id: Self::NId) -> Result<Self::N, GraphError> {
        if !self.contains_node(id) {
            return Err(GraphError::NodeNotFound);
        }

        self.clear_node(id)?;
        self.adj.unregister_node(id);
        Ok(self.nodes.remove_node(id).unwrap())
    }

    fn clear_node(&mut self, u: Self::NId) -> Result<(), GraphError> {
        let edge_ids: Vec<_> = self.adj.clear_node(u)?;
        for (edge_id, _) in edge_ids {
            self.edges
                .remove_edge(edge_id)
                .expect("Edge should be present");
        }
        Ok(())
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.adj.contains_adj(u, v)
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        self.edges.edge(id)
    }

    fn edge_mut(&mut self, id: Self::EId) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        self.edges.edge_mut(id)
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        let edge_id = self.adj.between(u, v)?;
        self.edges.edge(edge_id)
    }

    fn between_mut(
        &mut self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<EdgeMut<Self::NId, Self::EId, Self::E>> {
        let edge_id = self.adj.between(u, v)?;
        self.edges.edge_mut(edge_id)
    }

    fn insert_edge(
        &mut self,
        u: Self::NId,
        v: Self::NId,
        edge: Self::E,
    ) -> Result<Self::EId, GraphError> {
        if !self.contains_node(u) || !self.contains_node(v) {
            return Err(GraphError::NodeNotFound);
        }

        let edge_id = self.edges.insert_edge(u, v, edge);
        self.adj.insert_adj(u, v, edge_id);
        Ok(edge_id)
    }

    fn remove_edge(&mut self, id: Self::EId) -> Result<Self::E, GraphError> {
        let edge = self.edges.edge(id).ok_or(GraphError::EdgeNotFound)?;
        let (u, v) = (edge.u(), edge.v());
        self.adj.remove_adj(u, v, id);
        Ok(self.edges.remove_edge(id).unwrap())
    }

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a> {
        self.nodes.nodes()
    }

    fn nodes_mut<'a>(&'a mut self) -> Self::NodeMutIterator<'a> {
        self.nodes.nodes_mut()
    }

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a> {
        self.edges.edges()
    }

    fn edges_mut<'a>(&'a mut self) -> Self::EdgeMutIterator<'a> {
        self.edges.edges_mut()
    }

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        Some(GAdjIterator {
            graph: &self,
            inner: self.adj.adj(u)?,
        })
    }

    fn adj_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        Some(GAdjMutIterator {
            nodes: &mut self.nodes,
            edges: &mut self.edges,
            inner: self.adj.adj(u)?,
        })
    }
}

impl<NC, EC, AC> DirectedGraph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: DirectedAdjContainer<NId = NC::NId, EId = EC::EId>,
{
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        Some(GAdjIterator {
            graph: &self,
            inner: self.adj.out_adj(u)?,
        })
    }

    fn out_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        Some(GAdjMutIterator {
            nodes: &mut self.nodes,
            edges: &mut self.edges,
            inner: self.adj.out_adj(u)?,
        })
    }

    fn out_degree(&self, u: Self::NId) -> usize {
        self.adj.out_degree(u)
    }

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        Some(GAdjIterator {
            graph: &self,
            inner: self.adj.in_adj(u)?,
        })
    }

    fn in_edges_mut<'a>(&'a mut self, u: Self::NId) -> Option<Self::AdjMutIterator<'a>> {
        Some(GAdjMutIterator {
            nodes: &mut self.nodes,
            edges: &mut self.edges,
            inner: self.adj.in_adj(u)?,
        })
    }

    fn in_degree(&self, u: Self::NId) -> usize {
        self.adj.in_degree(u)
    }

    fn reverse_edge(&mut self, id: Self::EId) -> Result<(), GraphError> {
        let edge = self.edge(id).ok_or(GraphError::EdgeNotFound)?;
        self.adj.reverse_adj(edge.u(), edge.v(), id);

        self.edges.reverse_edge(id)
    }
}

impl<NC, EC, AC> UndirectedGraph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: UndirectedAdjContainer<NId = NC::NId, EId = EC::EId>,
{
}

impl<NC, EC, AC> RawGraph for CGraph<NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: RawAdjContainer<NId = NC::NId, EId = EC::EId>,
{
}

impl<NC, EC, AC> OrdinalGraph for CGraph<NC, EC, AC>
where
    NC: OrdinalNodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    fn insert_node(&mut self, node: Self::N) -> Self::NId {
        let id = self.nodes.insert_node(node);
        self.adj.register_node(id);
        id
    }

    fn from_ordinal(nodes: Vec<Self::N>, edges: Vec<(Self::NId, Self::NId, Self::E)>) -> Self
    where
        Self: WithCapacity,
    {
        let mut g = Self::with_capacity(nodes.len(), edges.len());

        for n in nodes {
            g.insert_node(n);
        }
        for (u, v, e) in edges {
            g.insert_edge(u, v, e)
                .expect("node ids should refer to valid nodes");
        }

        g
    }
}

impl<NC, EC, AC> KeyedGraph for CGraph<NC, EC, AC>
where
    NC: KeyedNodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: KeyedAdjContainer<NId = NC::NId, EId = EC::EId>,
{
    fn put_node(&mut self, id: Self::NId, node: Self::N) -> Option<Self::N> {
        if self.contains_node(id) {
            let node_data = self.node_mut(id).unwrap().into_data();
            return Some(std::mem::replace(node_data, node));
        }

        self.nodes.put_node(id, node);
        self.adj.register_node(id);
        None
    }

    fn from_keyed(
        nodes: Vec<(Self::NId, Self::N)>,
        edges: Vec<(Self::NId, Self::NId, Self::E)>,
    ) -> Self
    where
        Self: WithCapacity,
    {
        let mut g = Self::with_capacity(nodes.len(), edges.len());

        for (id, n) in nodes {
            g.put_node(id, n);
        }
        for (u, v, e) in edges {
            g.insert_edge(u, v, e)
                .expect("node ids should refer to valid nodes");
        }

        g
    }
}

impl<NC, EC, AC> Default for CGraph<NC, EC, AC>
where
    NC: Default,
    EC: Default,
    AC: Default,
{
    fn default() -> Self {
        Self {
            nodes: NC::default(),
            edges: EC::default(),
            adj: AC::default(),
        }
    }
}

impl<NC, EC, AC> WithCapacity for CGraph<NC, EC, AC>
where
    NC: WithCapacity,
    EC: WithCapacity,
    AC: WithCapacity,
{
    fn with_capacity(node_capacity: usize, edge_capacity: usize) -> Self {
        Self {
            nodes: NC::with_capacity(node_capacity, edge_capacity),
            edges: EC::with_capacity(node_capacity, edge_capacity),
            adj: AC::with_capacity(node_capacity, edge_capacity),
        }
    }
}

impl<NC, EC, AC> CGraph<NC, EC, AC>
where
    NC: Default,
    EC: Default,
    AC: Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct GAdjIterator<'a, NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    graph: &'a CGraph<NC, EC, AC>,
    inner: AC::AdjIterator<'a>,
}

impl<'a, NC, EC, AC> Iterator for GAdjIterator<'a, NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    type Item = (Edge<'a, NC::NId, EC::EId, EC::E>, Node<'a, NC::NId, NC::N>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(id, v)| {
            (
                self.graph
                    .edges
                    .edge(id)
                    .expect("id from adj iterator must refer to real edge"),
                self.graph
                    .nodes
                    .node(v)
                    .expect("id from adj iterator must refer to real edge"),
            )
        })
    }
}

pub struct GAdjMutIterator<'a, NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: 'a + AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    nodes: &'a mut NC,
    edges: &'a mut EC,
    inner: AC::AdjIterator<'a>,
}

impl<'a, NC, EC, AC> Iterator for GAdjMutIterator<'a, NC, EC, AC>
where
    NC: NodeContainer,
    EC: EdgeContainer<NId = NC::NId>,
    AC: 'a + AdjContainer<NId = NC::NId, EId = EC::EId>,
{
    type Item = (
        EdgeMut<'a, NC::NId, EC::EId, EC::E>,
        NodeMut<'a, NC::NId, NC::N>,
    );

    fn next(&mut self) -> Option<Self::Item> {
        let (edge_id, node_id) = self.inner.next()?;

        unsafe {
            let edges: *mut EC = self.edges;
            let nodes: *mut NC = self.nodes;
            Some((
                edges
                    .as_mut()?
                    .edge_mut(edge_id)
                    .expect("id from adj iterator must refer to real edge"),
                nodes
                    .as_mut()?
                    .node_mut(node_id)
                    .expect("id from adj iterator must refer to real edge"),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::graph::traits::{DirectedGraph, Graph, KeyedGraph, OrdinalGraph};
    use crate::graph::types::{
        DiFlatGraph, DiListGraph, DiMapGraph, UnFlatGraph, UnListGraph, UnMapGraph,
    };

    #[test]
    fn unmap_puts_and_removes() {
        // A --5-- B
        // |       |
        // 2       1
        // |       |
        // C --1-- D
        let mut graph = UnMapGraph::new();
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

        let mut a_adj: Vec<_> = graph
            .adj("A")
            .expect("A should have adj edges")
            .map(|(_, node)| node.id())
            .collect();
        a_adj.sort();
        assert_eq!(a_adj, vec!["B", "C"]);

        // edges should be undirected and the same id each direction
        assert!(graph.between("A", "B").is_some());
        assert!(graph.between("B", "A").is_some());
        assert_eq!(
            graph.between("A", "B").unwrap().id(),
            graph.between("B", "A").unwrap().id()
        );

        // removes A and adjacent edges
        graph.remove_node("A").expect("node should exist");
        assert!(graph.node("A").is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.degree("B"), 1);
        assert_eq!(graph.degree("C"), 1);
        assert_eq!(graph.degree("D"), 2);

        // single deleted edge should be gone
        let edge_id = graph.between("D", "C").unwrap().id();
        graph.remove_edge(edge_id);

        assert_eq!(graph.degree("C"), 0);
        assert_eq!(graph.degree("D"), 1);
    }

    #[test]
    fn dimap_puts_and_removes() {
        // A --5-> B
        // |       |
        // 2       1
        // v       v
        // C --1-> D
        let mut graph = DiMapGraph::new();
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

        let mut a_out: Vec<_> = graph
            .out_edges("A")
            .expect("A should have out edges")
            .map(|(_, node)| node.id())
            .collect();
        a_out.sort();
        assert_eq!(a_out, vec!["B", "C"]);

        let mut d_in: Vec<_> = graph
            .in_edges("D")
            .expect("D should have in edges")
            .map(|(_, node)| node.id())
            .collect();
        d_in.sort();
        assert_eq!(d_in, vec!["B", "C"]);

        // edges should be directed
        assert!(graph.between("A", "B").is_some());
        assert!(graph.between("B", "A").is_none());

        // remove A and adjacent edges
        graph.remove_node("A").expect("node should exist");
        assert!(graph.node("A").is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.in_degree("B"), 0);
        assert_eq!(graph.out_degree("B"), 1);
        assert_eq!(graph.in_degree("C"), 0);
        assert_eq!(graph.out_degree("C"), 1);
        assert_eq!(graph.in_degree("D"), 2);
        assert_eq!(graph.out_degree("D"), 0);

        // remove D and adjacent edges (including incoming edges)
        graph.remove_node("D").expect("node should exist");
        let (n3, e3) = graph.len();
        assert_eq!(n3, 2);
        assert_eq!(e3, 0);

        assert_eq!(graph.out_degree("B"), 0);
        assert_eq!(graph.in_degree("B"), 0);
        assert_eq!(graph.out_degree("C"), 0);
        assert_eq!(graph.in_degree("C"), 0);
    }

    #[test]
    fn unlist_puts_and_removes() {
        // N0 --5-- N1
        // |         |
        // 2         1
        // |         |
        // N2 --1-- N3
        let mut graph = UnListGraph::new();
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_edge(0, 1, 5).expect("nodes should exist");
        graph.insert_edge(0, 2, 2).expect("nodes should exist");
        graph.insert_edge(2, 3, 1).expect("nodes should exist");
        graph.insert_edge(1, 3, 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        let mut a_adj: Vec<_> = graph
            .adj(0)
            .expect("A should have adj edges")
            .map(|(_, node)| node.id())
            .collect();
        a_adj.sort();
        assert_eq!(a_adj, vec![1, 2]);

        // edges should be undirected and the same id each direction
        assert!(graph.between(0, 1).is_some());
        assert!(graph.between(1, 0).is_some());
        assert_eq!(
            graph.between(0, 1).unwrap().id(),
            graph.between(1, 0).unwrap().id()
        );

        // removes A and adjacent edges
        graph.remove_node(0).expect("node should exist");
        assert!(graph.node(0).is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.degree(1), 1);
        assert_eq!(graph.degree(2), 1);
        assert_eq!(graph.degree(3), 2);

        // single deleted edge should be gone
        let edge_id = graph.between(3, 2).unwrap().id();
        graph.remove_edge(edge_id);

        assert_eq!(graph.degree(2), 0);
        assert_eq!(graph.degree(3), 1);
    }

    #[test]
    fn dilist_puts_and_removes() {
        // N0 --5-> N1
        // |         |
        // 2         1
        // v         v
        // N2 --1-> N3
        let mut graph = DiListGraph::new();
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_edge(0, 1, 5).expect("nodes should exist");
        graph.insert_edge(0, 2, 2).expect("nodes should exist");
        graph.insert_edge(2, 3, 1).expect("nodes should exist");
        graph.insert_edge(1, 3, 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        let mut a_out: Vec<_> = graph
            .out_edges(0)
            .expect("N0 should have out edges")
            .map(|(_, node)| node.id())
            .collect();
        a_out.sort();
        assert_eq!(a_out, vec![1, 2]);

        let mut d_in: Vec<_> = graph
            .in_edges(3)
            .expect("N3 should have in edges")
            .map(|(_, node)| node.id())
            .collect();
        d_in.sort();
        assert_eq!(d_in, vec![1, 2]);

        // edges should be directed
        assert!(graph.between(0, 1).is_some());
        assert!(graph.between(1, 0).is_none());

        // remove A and adjacent edges
        graph.remove_node(0).expect("node should exist");
        assert!(graph.node(0).is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.in_degree(1), 0);
        assert_eq!(graph.out_degree(1), 1);
        assert_eq!(graph.in_degree(2), 0);
        assert_eq!(graph.out_degree(2), 1);
        assert_eq!(graph.in_degree(3), 2);
        assert_eq!(graph.out_degree(3), 0);

        // remove D and adjacent edges (including incoming edges)
        graph.remove_node(3).expect("node should exist");
        let (n3, e3) = graph.len();
        assert_eq!(n3, 2);
        assert_eq!(e3, 0);

        assert_eq!(graph.out_degree(1), 0);
        assert_eq!(graph.in_degree(1), 0);
        assert_eq!(graph.out_degree(2), 0);
        assert_eq!(graph.in_degree(2), 0);
    }

    #[test]
    fn unmap_iteration() {
        // A ----- B
        // |  \ /  |
        // |   X   |
        // |  / \  |
        // C ----- D
        let mut graph = UnMapGraph::from_keyed(
            vec![("A", 0), ("B", 0), ("C", 0), ("D", 0)],
            vec![
                ("A", "B", 2),
                ("B", "C", 2),
                ("C", "D", 2),
                ("A", "D", 2),
                ("A", "C", 2),
                ("D", "B", 2),
            ],
        );

        for mut node in graph.nodes_mut() {
            assert_eq!(*node, 0);
            *node = 10;
        }

        for node in graph.nodes() {
            assert_eq!(*node, 10);
        }

        for mut edge in graph.edges_mut() {
            assert_eq!(*edge, 2);
            *edge = 1;
        }

        for edge in graph.edges() {
            assert_eq!(*edge, 1);
        }

        let mut a_adj: Vec<_> = graph
            .adj("A")
            .expect("A should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        a_adj.sort();
        assert_eq!(a_adj, vec![("B", 1), ("C", 1), ("D", 1)]);

        let mut b_adj: Vec<_> = graph
            .adj("B")
            .expect("B should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        b_adj.sort();
        assert_eq!(b_adj, vec![("A", 1), ("C", 1), ("D", 1)]);

        let mut c_adj: Vec<_> = graph
            .adj("C")
            .expect("C should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        c_adj.sort();
        assert_eq!(c_adj, vec![("A", 1), ("B", 1), ("D", 1)]);

        let mut d_adj: Vec<_> = graph
            .adj("D")
            .expect("D should have adj edges")
            .map(|(edge, node)| (node.id(), edge.data().clone()))
            .collect();
        d_adj.sort();
        assert_eq!(d_adj, vec![("A", 1), ("B", 1), ("C", 1)]);

        *graph.node_mut("A").unwrap() = 3;
        assert_eq!(*graph.node("A").unwrap(), 3);

        *graph.edge_mut(0).unwrap() = 5;
        assert_eq!(*graph.edge(0).unwrap(), 5);
        *graph.edge_mut(0).unwrap() = 1;

        for (mut edge_mut, mut node_mut) in graph.adj_mut("A").unwrap() {
            assert_eq!(*edge_mut, 1);
            assert_eq!(*node_mut, 10);
            *edge_mut = 2;
            *node_mut = 0;
        }

        for (edge, node) in graph.adj("A").unwrap() {
            assert_eq!(*edge, 2);
            assert_eq!(*node, 0);
        }
    }

    #[test]
    fn unflat_puts_and_removes() {
        // N0 --5-- N1
        // |         |
        // 2         1
        // |         |
        // N2 --1-- N3
        let mut graph = UnFlatGraph::new();
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_edge(0, 1, 5).expect("nodes should exist");
        graph.insert_edge(0, 2, 2).expect("nodes should exist");
        graph.insert_edge(2, 3, 1).expect("nodes should exist");
        graph.insert_edge(1, 3, 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        let mut a_adj: Vec<_> = graph
            .adj(0)
            .expect("A should have adj edges")
            .map(|(_, node)| node.id())
            .collect();
        a_adj.sort();
        assert_eq!(a_adj, vec![1, 2]);

        // edges should be undirected and the same id each direction
        assert!(graph.between(0, 1).is_some());
        assert!(graph.between(1, 0).is_some());
        assert_eq!(
            graph.between(0, 1).unwrap().id(),
            graph.between(1, 0).unwrap().id()
        );

        // removes A and adjacent edges
        graph.remove_node(0).expect("node should exist");
        assert!(graph.node(0).is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.degree(1), 1);
        assert_eq!(graph.degree(2), 1);
        assert_eq!(graph.degree(3), 2);

        // single deleted edge should be gone
        let edge_id = graph.between(3, 2).unwrap().id();
        graph.remove_edge(edge_id);

        assert_eq!(graph.degree(2), 0);
        assert_eq!(graph.degree(3), 1);
    }

    #[test]
    fn diflat_puts_and_removes() {
        // N0 --5-> N1
        // |         |
        // 2         1
        // v         v
        // N2 --1-> N3
        let mut graph = DiFlatGraph::new();
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_node(());
        graph.insert_edge(0, 1, 5).expect("nodes should exist");
        graph.insert_edge(0, 2, 2).expect("nodes should exist");
        graph.insert_edge(2, 3, 1).expect("nodes should exist");
        graph.insert_edge(1, 3, 1).expect("nodes should exist");

        let (n, e) = graph.len();
        assert_eq!(n, 4);
        assert_eq!(e, 4);

        let mut a_out: Vec<_> = graph
            .out_edges(0)
            .expect("N0 should have out edges")
            .map(|(_, node)| node.id())
            .collect();
        a_out.sort();
        assert_eq!(a_out, vec![1, 2]);

        let mut d_in: Vec<_> = graph
            .in_edges(3)
            .expect("N3 should have in edges")
            .map(|(_, node)| node.id())
            .collect();
        d_in.sort();
        assert_eq!(d_in, vec![1, 2]);

        // edges should be directed
        assert!(graph.between(0, 1).is_some());
        assert!(graph.between(1, 0).is_none());

        // remove A and adjacent edges
        graph.remove_node(0).expect("node should exist");
        assert!(graph.node(0).is_none());

        let (n2, e2) = graph.len();
        assert_eq!(n2, 3);
        assert_eq!(e2, 2);

        assert_eq!(graph.in_degree(1), 0);
        assert_eq!(graph.out_degree(1), 1);
        assert_eq!(graph.in_degree(2), 0);
        assert_eq!(graph.out_degree(2), 1);
        assert_eq!(graph.in_degree(3), 2);
        assert_eq!(graph.out_degree(3), 0);

        assert!(graph.contains_edge(1, 3));
        assert!(!graph.contains_edge(0, 1));

        // remove D and adjacent edges (including incoming edges)
        graph.remove_node(3).expect("node should exist");
        let (n3, e3) = graph.len();
        assert_eq!(n3, 2);
        assert_eq!(e3, 0);

        assert_eq!(graph.out_degree(1), 0);
        assert_eq!(graph.in_degree(1), 0);
        assert_eq!(graph.out_degree(2), 0);
        assert_eq!(graph.in_degree(2), 0);

        graph.insert_edge(1, 2, 1).expect("nodes should exist");
        graph.insert_edge(2, 1, 1).expect("nodes should exist");
        assert_eq!(graph.out_degree(1), 1);
        assert_eq!(graph.in_degree(1), 1);
        assert_eq!(graph.out_degree(2), 1);
        assert_eq!(graph.in_degree(2), 1);
        assert_eq!(graph.len(), (2, 2));
    }
}
