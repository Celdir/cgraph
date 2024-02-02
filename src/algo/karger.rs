use disjoint::DisjointSet;
use itertools::Itertools;
use rand::seq::SliceRandom;

use crate::graph::edge::Edge;

use crate::graph::traits::{GraphIter, UndirectedGraph};
use crate::graph::types::NodeHashMap;

use std::ops::{Add, Sub};
use std::rc::Rc;

pub fn karger<'a, G>(graph: &'a G) -> Contraction<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    let mut ids = NodeHashMap::<G, usize>::default();
    let mut counter = 0;
    for node in graph.nodes() {
        ids.insert(node.id(), counter);
        counter += 1;
    }
    let mut c = Contraction::new(graph, ids);
    fastmincut(&mut c)
}

fn fastmincut<'a, G>(c: &mut Contraction<'a, G>) -> Contraction<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    if c.components <= 6 {
        c.contract_to(2);
        return c.clone();
    }
    let t = (1.0 + (c.components as f64) / 2.0_f64.sqrt()).ceil() as usize;
    let mut c2 = c.clone();
    c.contract_to(t);
    c2.contract_to(t);
    let r1 = fastmincut(c);
    let r2 = fastmincut(&mut c2);
    if r1.cut_weight < r2.cut_weight {
        return r1;
    } else {
        return r2;
    }
}

pub struct Contraction<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    graph: &'a G,
    ids: Rc<NodeHashMap<G, usize>>,
    ds: DisjointSet,
    components: usize,
    cut_weight: G::E,
}

impl<'a, G> Contraction<'a, G>
where
    G: UndirectedGraph + GraphIter,
    G::E: Default + Add<Output = G::E> + Sub<Output = G::E> + Ord + Clone,
{
    pub fn new(g: &'a G, ids: NodeHashMap<G, usize>) -> Contraction<'a, G> {
        let (n, _) = g.len();
        Contraction {
            graph: g,
            ids: ids.into(),
            ds: DisjointSet::with_len(n),
            components: n,
            cut_weight: g
                .edges()
                .fold(G::E::default(), |acc, e| acc + e.data().clone()),
        }
    }

    pub fn is_contracted(&self, edge: &Edge<'a, G::NId, G::EId, G::E>) -> bool {
        self.ds.is_joined(self.ids[&edge.u()], self.ids[&edge.v()])
    }

    pub fn contract(&mut self, edge: &Edge<'a, G::NId, G::EId, G::E>) {
        if self.ds.join(self.ids[&edge.u()], self.ids[&edge.v()]) {
            self.components -= 1;
            self.cut_weight = self.cut_weight.clone() - edge.data().clone();
        }
    }

    pub fn contract_to(&mut self, k: usize) {
        if self.components <= k {
            return;
        }

        let mut rng = rand::thread_rng();
        let edges = self.cut_edges();
        for edge in edges.choose_multiple(&mut rng, self.components - k) {
            self.contract(edge);
        }
    }

    pub fn cut_weight(&self) -> G::E {
        self.cut_weight.clone()
    }

    pub fn cut_edges(&self) -> Vec<Edge<'a, G::NId, G::EId, G::E>> {
        self.graph
            .edges()
            .filter(|e| !self.is_contracted(e))
            .collect_vec()
    }
}

impl<'a, G> Clone for Contraction<'a, G>
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
            cut_weight: self.cut_weight.clone(),
        }
    }
}
