use ahash::AHashSet;
use dary_heap::DaryHeap;
use itertools::Itertools;

use crate::graph::edge::Edge;

use crate::graph::traits::{GraphIter, UndirectedGraph};
use crate::graph::types::NodeHashMap;
use crate::utils::disjoint_sets::DisjointSet;

use std::ops::{Add, Sub};
use std::rc::Rc;

// Stoer Wagner
pub fn mincut<'a, G>(graph: &'a G) -> Quotient<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    let mut q = Quotient::new(graph);
    let mut best = q.clone();
    while q.components() > 1 {
        let (st_cut, s, t) = phase(q.clone());
        if st_cut.cut_weight() < best.cut_weight() {
            best = st_cut;
        }
        q.merge_subsets(s, t);
    }
    best
}

fn phase<'a, G>(mut quotient: Quotient<'a, G>) -> (Quotient<'a, G>, usize, usize)
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    let start = quotient.graph().nodes().next().unwrap().id();
    let start_id = quotient.subset(start);
    let mut heap: DaryHeap<(G::E, usize), 4> = DaryHeap::new();
    let mut weight = vec![G::E::default(); quotient.graph().len().0];
    for edge in quotient.adj(start_id) {
        let other = edge.other(start_id);
        weight[other] = weight[other].clone() + edge.data().clone();
        heap.push((weight[other].clone(), other));
    }

    let mut s = start_id;
    let mut t = s;
    while !heap.is_empty() {
        let (_, i) = heap.pop().unwrap();
        if quotient.is_merged(start_id, i) {
            continue;
        }
        s = t;
        t = i;
        if quotient.components() <= 2 {
            break;
        }

        let i_root = quotient.root(i);
        let new_edges = quotient.adj(i_root).into_iter().filter(|edge| {
            if i_root == edge.u() {
                edge.v() != start_id
            } else {
                edge.u() != start_id
            }
        });
        for edge in new_edges {
            let (u, v) = (edge.u(), edge.v());
            let w = edge.data().clone();
            let other = if u != i_root { u } else { v };
            let o_root = quotient.root(other);
            weight[o_root] = weight[o_root].clone() + w;
            heap.push((weight[o_root].clone(), o_root));
        }
        quotient.merge_subsets(start_id, i);
    }
    (quotient, s, t)
}

pub struct Partition<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    graph: &'a G,
    ids: Rc<NodeHashMap<G, usize>>,
    ds: DisjointSet,
    components: usize,
}

impl<'a, G> Partition<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    pub fn new(g: &'a G) -> Partition<'a, G> {
        let mut ids = NodeHashMap::<G, usize>::default();
        let mut counter = 0;
        for node in g.nodes() {
            ids.insert(node.id(), counter);
            counter += 1;
        }

        let (n, _) = g.len();
        Partition {
            graph: g,
            ids: ids.into(),
            ds: DisjointSet::with_len(n),
            components: n,
        }
    }

    pub fn is_contracted(&self, edge: &Edge<'a, G::NId, G::EId, G::E>) -> bool {
        self.ds.same_set(self.ids[&edge.u()], self.ids[&edge.v()])
    }

    pub fn is_merged(&self, u: usize, v: usize) -> bool {
        self.ds.same_set(u, v)
    }

    pub fn same_subset(&self, u: G::NId, v: G::NId) -> bool {
        self.ds.same_set(self.ids[&u], self.ids[&v])
    }

    pub fn subset(&self, u: G::NId) -> usize {
        self.root(self.ids[&u])
    }

    pub fn contract_edge(&mut self, edge: &Edge<'a, G::NId, G::EId, G::E>) -> bool {
        self.merge_subsets(self.ids[&edge.u()], self.ids[&edge.v()])
    }

    pub fn merge_subsets(&mut self, a: usize, b: usize) -> bool {
        let contracted = self.ds.union(a, b);
        if contracted {
            self.components -= 1;
        }
        contracted
    }

    pub fn root(&self, id: usize) -> usize {
        self.ds.root(id)
    }

    pub fn cut_weight(&self) -> G::E {
        self.cut_edges()
            .into_iter()
            .fold(G::E::default(), |acc, e| acc + e.data().clone())
    }

    pub fn cut_edges(&self) -> Vec<Edge<'a, G::NId, G::EId, G::E>> {
        self.graph
            .edges()
            .filter(|e| !self.is_contracted(e))
            .collect_vec()
    }

    pub fn components(&self) -> usize {
        self.components
    }

    pub fn graph(&'a self) -> &'a G {
        self.graph
    }
}

impl<'a, G> Clone for Partition<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    fn clone(&self) -> Self {
        Self {
            graph: self.graph,
            ids: self.ids.clone(),
            ds: self.ds.clone(),
            components: self.components,
        }
    }
}

pub struct Quotient<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    partition: Partition<'a, G>,
    edges: Vec<AHashSet<G::EId>>,
}

impl<'a, G> Quotient<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    pub fn new(g: &'a G) -> Quotient<'a, G> {
        let partition = Partition::new(g);
        let mut edges = vec![AHashSet::new(); g.len().0];
        for edge in g.edges() {
            edges[partition.subset(edge.u())].insert(edge.id());
            edges[partition.subset(edge.v())].insert(edge.id());
        }
        Quotient { partition, edges }
    }

    pub fn is_contracted(&self, edge: &Edge<'a, G::NId, G::EId, G::E>) -> bool {
        self.partition.is_contracted(edge)
    }

    pub fn is_merged(&self, u: usize, v: usize) -> bool {
        self.partition.is_merged(u, v)
    }

    pub fn same_subset(&self, u: G::NId, v: G::NId) -> bool {
        self.partition.same_subset(u, v)
    }

    pub fn subset(&self, u: G::NId) -> usize {
        self.partition.subset(u)
    }

    pub fn contract_edge(&mut self, edge: &Edge<'a, G::NId, G::EId, G::E>) -> bool {
        let u_root = self.partition.subset(edge.u());
        let v_root = self.partition.subset(edge.v());
        self.merge_subsets(u_root, v_root)
    }

    pub fn merge_subsets(&mut self, u: usize, v: usize) -> bool {
        let u_root = self.partition.root(u);
        let v_root = self.partition.root(v);
        let merged = self.partition.merge_subsets(u_root, v_root);
        if merged {
            let new_root = self.partition.ds.root(u_root);
            if new_root == u_root {
                self.merge_edges(v_root, new_root);
            } else {
                self.merge_edges(u_root, new_root);
            }
        }
        merged
    }

    fn merge_edges(&mut self, old_root: usize, new_root: usize) {
        let inter = self.edges[old_root]
            .intersection(&self.edges[new_root])
            .copied()
            .collect_vec();
        for edge_id in inter {
            self.edges[old_root].remove(&edge_id);
            self.edges[new_root].remove(&edge_id);
        }
        let moved = self.edges[old_root].drain().collect_vec();
        self.edges[new_root].extend(moved);
    }

    pub fn root(&self, id: usize) -> usize {
        self.partition.root(id)
    }

    pub fn cut_weight(&self) -> G::E {
        self.partition.cut_weight()
    }

    pub fn cut_edges(&self) -> Vec<Edge<'a, G::NId, G::EId, G::E>> {
        self.partition.cut_edges()
    }

    pub fn adj(&self, id: usize) -> Vec<Edge<'a, usize, G::EId, G::E>> {
        let root = self.root(id);
        self.edges[root]
            .iter()
            .map(|&edge_id| self.partition.graph.edge(edge_id).unwrap())
            .map(|edge| {
                Edge::from_value(
                    edge.id(),
                    self.subset(edge.u()),
                    self.subset(edge.v()),
                    edge.data().clone(),
                )
            })
            .collect_vec()
    }

    pub fn components(&self) -> usize {
        self.partition.components()
    }

    pub fn graph(&'a self) -> &'a G {
        self.partition.graph()
    }
}

impl<'a, G> Clone for Quotient<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    fn clone(&self) -> Self {
        Self {
            partition: self.partition.clone(),
            edges: self.edges.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{algo::min_cut::mincut, graph::builder::GraphBuilder};

    #[test]
    fn basic_case() {
        let graph = GraphBuilder::<(), usize>::new()
            .adj_flat()
            .undirected()
            .ordinal()
            .with_size(5)
            .edge(0, 1, 1)
            .edge(1, 2, 1)
            .edge(0, 3, 1)
            .edge(0, 4, 1)
            .edge(1, 4, 1)
            .edge(3, 4, 1)
            .edge(2, 4, 1)
            .build();
        let contraction = mincut(&graph);
        assert_eq!(contraction.cut_weight(), 2);
        assert_eq!(contraction.components(), 2);
    }
}
