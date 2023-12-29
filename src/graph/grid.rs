use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Add, Range},
};

use itertools::{Itertools, Product};

use super::{
    edge::Edge,
    node::Node,
    traits::{DirectedGraph, Graph, UndirectedGraph},
};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub struct Position(isize, isize);

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, d: Direction) -> Position {
        let (i, j) = (self.0, self.1);
        match d {
            Direction::Up => Position(i - 1, j),
            Direction::Down => Position(i + 1, j),
            Direction::Left => Position(i, j - 1),
            Direction::Right => Position(i, j + 1),
            Direction::UpLeft => Position(i - 1, j - 1),
            Direction::UpRight => Position(i - 1, j + 1),
            Direction::DownLeft => Position(i + 1, j - 1),
            Direction::DownRight => Position(i + 1, j + 1),
        }
    }
}

impl From<(isize, isize)> for Position {
    fn from(value: (isize, isize)) -> Self {
        Position(value.0, value.1)
    }
}

pub struct Bounds {
    i: Option<Range<isize>>,
    j: Option<Range<isize>>,
}

impl Bounds {
    pub fn new(i: Range<isize>, j: Range<isize>) -> Bounds {
        Bounds {
            i: Some(i),
            j: Some(j),
        }
    }

    pub fn unbounded() -> Bounds {
        Bounds { i: None, j: None }
    }

    fn check(&self, p: Position) -> bool {
        let check_i = match &self.i {
            Some(range) => range.contains(&p.0),
            _ => true,
        };
        let check_j = match &self.j {
            Some(range) => range.contains(&p.1),
            _ => true,
        };
        check_i && check_j
    }

    fn len(&self) -> (usize, usize) {
        (
            self.i
                .as_ref()
                .map(|range| range.len())
                .unwrap_or(usize::MAX),
            self.j
                .as_ref()
                .map(|range| range.len())
                .unwrap_or(usize::MAX),
        )
    }

    fn iter(&self) -> Product<Range<isize>, Range<isize>> {
        let irange = self.i.clone().unwrap_or(isize::MIN..isize::MAX);
        let jrange = self.j.clone().unwrap_or(isize::MIN..isize::MAX);
        irange.cartesian_product(jrange)
    }
}

pub struct Grid2D<N, E, NV, EV, T>
where
    NV: Fn(Position) -> N,
    EV: Fn((Position, Position)) -> E,
    T: Fn(Position) -> Vec<Position>,
{
    bounds: Bounds,
    node_val: NV,
    edge_val: EV,
    transition: T,
}

pub struct GridMaker {}

impl GridMaker {
    pub fn four_connected<N, E, NV, EV, F>(
        bounds: Bounds,
        node_val: NV,
        edge_val: EV,
        filter: F,
    ) -> Grid2D<N, E, NV, EV, impl Fn(Position) -> Vec<Position>>
    where
        NV: Fn(Position) -> N,
        EV: Fn((Position, Position)) -> E,
        F: Fn(Position, Position) -> bool,
    {
        Grid2D {
            bounds,
            node_val,
            edge_val,
            transition: move |pos: Position| {
                let mut neighbors = Vec::new();
                for direction in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    let adj = pos + direction;
                    if (filter)(pos, adj) {
                        neighbors.push(adj);
                    }
                }
                neighbors
            },
        }
    }

    pub fn eight_connected<N, E, NV, EV, F>(
        bounds: Bounds,
        node_val: NV,
        edge_val: EV,
        filter: F,
    ) -> Grid2D<N, E, NV, EV, impl Fn(Position) -> Vec<Position>>
    where
        NV: Fn(Position) -> N,
        EV: Fn((Position, Position)) -> E,
        F: Fn(Position, Position) -> bool,
    {
        Grid2D {
            bounds,
            node_val,
            edge_val,
            transition: move |pos: Position| {
                let mut neighbors = Vec::new();
                for direction in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                    Direction::UpLeft,
                    Direction::UpRight,
                    Direction::DownLeft,
                    Direction::DownRight,
                ] {
                    let adj = pos + direction;
                    if (filter)(pos, adj) {
                        neighbors.push(adj);
                    }
                }
                neighbors
            },
        }
    }
}

impl<N, E, NV, EV, T> Graph for Grid2D<N, E, NV, EV, T>
where
    NV: Fn(Position) -> N,
    EV: Fn((Position, Position)) -> E,
    T: Fn(Position) -> Vec<Position>,
{
    type NId = Position;
    type N = N;
    type EId = (Position, Position);
    type E = E;
    type NodeIterator<'a> = NodeIterator<'a, N, NV> where Self: 'a;
    type EdgeIterator<'a> = EdgeIterator<'a, Self> where Self: 'a;
    type AdjIterator<'a> = std::vec::IntoIter<
        (
            Edge<'a, Self::NId, Self::EId, Self::E>,
            Node<'a, Self::NId, Self::N>,
        ),
    > where Self: 'a;
    type AdjIdsIterator<'a> = std::vec::IntoIter<(Self::EId, Self::NId)> where Self: 'a;

    fn len(&self) -> (usize, usize) {
        self.bounds.len()
    }

    fn contains_node(&self, id: Self::NId) -> bool {
        self.bounds.check(id)
    }
    fn node(&self, id: Self::NId) -> Option<Node<Self::NId, Self::N>> {
        if !self.contains_node(id) {
            return None;
        }
        let val = (self.node_val)(id);
        Some(Node::from_value(id, val))
    }
    fn degree(&self, u: Self::NId) -> usize {
        (self.transition)(u).len()
    }

    fn contains_edge(&self, u: Self::NId, v: Self::NId) -> bool {
        if !self.bounds.check(u) || !self.bounds.check(v) {
            return false;
        }
        (self.transition)(u)
            .into_iter()
            .find(|&pos| pos == v)
            .is_some()
    }

    fn edge(&self, id: Self::EId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        if !self.contains_edge(id.0, id.1) {
            return None;
        }
        let val = (self.edge_val)(id);
        Some(Edge::from_value(id, id.0, id.1, val))
    }
    fn between(&self, u: Self::NId, v: Self::NId) -> Option<Edge<Self::NId, Self::EId, Self::E>> {
        self.edge((u, v))
    }

    fn nodes<'a>(&'a self) -> Self::NodeIterator<'a> {
        NodeIterator {
            inner: self.bounds.iter(),
            node_val: &self.node_val,
        }
    }

    fn edges<'a>(&'a self) -> Self::EdgeIterator<'a> {
        EdgeIterator {
            g: &self,
            inner: self.nodes(),
            adj: vec![].into_iter(),
        }
    }

    fn adj<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIterator<'a>> {
        Some(
            self.adj_ids(u)?
                .map(|(eid, nid)| (self.edge(eid), self.node(nid)))
                .filter(|(edge, node)| edge.is_some() && node.is_some())
                .map(|(e_opt, n_opt)| (e_opt.unwrap(), n_opt.unwrap()))
                .collect_vec()
                .into_iter(),
        )
    }

    fn adj_ids<'a>(&'a self, u: Self::NId) -> Option<Self::AdjIdsIterator<'a>> {
        if !self.contains_node(u) {
            return None;
        }
        Some(
            (self.transition)(u)
                .into_iter()
                .filter(|v| self.bounds.check(*v))
                .map(|v| ((u, v), v))
                .collect_vec()
                .into_iter(),
        )
    }
}

pub struct NodeIterator<'a, N, NV>
where
    N: 'a,
    NV: Fn(Position) -> N,
{
    inner: Product<Range<isize>, Range<isize>>,
    node_val: &'a NV,
}

impl<'a, N, NV> Iterator for NodeIterator<'a, N, NV>
where
    N: 'a,
    NV: Fn(Position) -> N,
{
    type Item = Node<'a, Position, N>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.inner.next()?.into();
        let val = (self.node_val)(pos);
        Some(Node::from_value(pos, val))
    }
}

pub struct EdgeIterator<'a, G>
where
    G: Graph<NId = Position, EId = (Position, Position)>,
{
    g: &'a G,
    inner: G::NodeIterator<'a>,
    adj: std::vec::IntoIter<G::EId>,
}

impl<'a, G> Iterator for EdgeIterator<'a, G>
where
    G: Graph<NId = Position, EId = (Position, Position)>,
{
    type Item = Edge<'a, G::NId, G::EId, G::E>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let eid = self.adj.next();
            if eid.is_some() {
                return self.g.edge(eid.unwrap());
            }
            self.adj = self
                .g
                .adj_ids(self.inner.next()?.id())
                .unwrap()
                .map(|(eid, _)| eid)
                .collect_vec()
                .into_iter();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::bfs::bfs;

    use super::{Bounds, GridMaker, Position};

    #[test]
    fn four_connected_bfs() {
        let g = GridMaker::four_connected(Bounds::new(-1..2, -1..2), |_| (), |_| 1, |_, _| true);
        let visited: Vec<_> = bfs(&g, Position(0, 0)).map(|(_, node)| node.id()).collect();
        assert_eq!(
            visited,
            vec![
                Position(0, 0),
                Position(-1, 0),
                Position(1, 0),
                Position(0, -1),
                Position(0, 1),
                Position(-1, -1),
                Position(-1, 1),
                Position(1, -1),
                Position(1, 1)
            ]
        );
    }
}
