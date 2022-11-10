use crate::graph::containers::adj::traits::{
    AdjContainer, DirectedAdjContainer, KeyedAdjContainer, MultiAdjContainer, OrdinalAdjContainer,
};

pub struct Di<AC: AdjContainer> {
    out_adj: AC,
    in_adj: AC,
}

impl<AC: AdjContainer> AdjContainer for Di<AC> {
    type NId = AC::NId;
    type EId = AC::EId;

    type AdjIterator<'a> = AC::AdjIterator<'a>
    where
        Self: 'a;

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.out_adj.adj(u)
    }

    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Self::EId> {
        self.out_adj.between(u, v)
    }

    fn degree(&self, u: Self::NId) -> usize {
        self.out_adj.degree(u)
    }

    fn insert_node(&mut self, u: Self::NId) {
        self.out_adj.insert_node(u);
        self.in_adj.insert_node(u);
    }

    fn remove_node(&mut self, u: Self::NId) {
        self.out_adj.remove_node(u);
        self.in_adj.remove_node(u);
    }

    fn clear_node(&mut self, u: Self::NId) -> Option<Vec<(Self::NId, Self::EId)>> {
        // TODO: how should this work at the Graph level? when graph iterates over adj() to
        // determine the edges to remove, it doesn't look at in edges, but here we clear both.
        // Should clear_node in adj container actually remove the adjacencies from neighboring
        // nodes as well (currently being done at Graph level) and return a vec of edge ids to
        // Graph can remove those from the edge container? I think this is the best option.
        let mut out_ids = self.out_adj.clear_node(u)?;
        let mut in_ids = self
            .in_adj
            .clear_node(u)
            .expect("out_adj and in_adj should both have the same nodes");
        out_ids.append(&mut in_ids);

        Some(out_ids)
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        self.out_adj.contains_edge(u, v)
    }

    fn insert_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.out_adj.insert_edge(u, v, edge_id);
        self.in_adj.insert_edge(v, u, edge_id);
    }

    fn remove_edge(&mut self, u: Self::NId, v: Self::NId, edge_id: Self::EId) {
        self.out_adj.remove_edge(u, v, edge_id);
        self.in_adj.remove_edge(v, u, edge_id);
    }
}

impl<AC: AdjContainer> DirectedAdjContainer for Di<AC> {
    fn out_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.out_adj.adj(u)
    }

    fn out_degree(&self, u: Self::NId) -> usize {
        self.out_adj.degree(u)
    }

    fn in_edges<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        self.in_adj.adj(u)
    }

    fn in_degree(&self, u: Self::NId) -> usize {
        self.in_adj.degree(u)
    }

    fn reverse_edge(&mut self, u: Self::NId, v: Self::NId, id: Self::EId) {
        self.out_adj.remove_edge(u, v, id);
        self.in_adj.remove_edge(v, u, id);

        self.out_adj.insert_edge(v, u, id);
        self.in_adj.insert_edge(u, v, id);
    }
}

impl<AC> MultiAdjContainer for Di<AC>
where
    AC: MultiAdjContainer,
{
    type MultiEdgeIterator<'a> = AC::MultiEdgeIterator<'a> where Self: 'a;

    fn between_multi<'a>(
        &'a self,
        u: Self::NId,
        v: Self::NId,
    ) -> Option<Self::MultiEdgeIterator<'a>> {
        self.out_adj.between_multi(u, v)
    }
}

impl<AC> KeyedAdjContainer for Di<AC> where AC: KeyedAdjContainer {}
impl<AC> OrdinalAdjContainer for Di<AC> where AC: OrdinalAdjContainer {}
