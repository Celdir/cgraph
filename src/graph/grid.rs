use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Add, Range},
};

use itertools::{Itertools, Product};

use super::{state::StateGraph};

#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

pub struct Grid {}

impl Grid {
    pub fn four_connected<State, N, E, NV, EV, NF, EF>(
        node_val: NV,
        edge_val: EV,
        node_filter: NF,
        edge_filter: EF,
    ) -> StateGraph<State, N, E, NV, EV, impl Fn(State) -> Vec<State>, impl Fn(State) -> bool>
    where
        State: Eq + Hash + Copy + Debug + Add<Direction, Output = State>,
        NV: Fn(State) -> N,
        EV: Fn((State, State)) -> E,
        NF: Fn(State) -> bool,
        EF: Fn(State, State) -> bool,
    {
        StateGraph::new(
            node_val,
            edge_val,
            move |state: State| {
                let mut neighbors = Vec::new();
                for direction in [
                    Direction::Up,
                    Direction::Down,
                    Direction::Left,
                    Direction::Right,
                ] {
                    let adj = state + direction;
                    if (edge_filter)(state, adj) {
                        neighbors.push(adj);
                    }
                }
                neighbors
            },
            node_filter,
        )
    }

    pub fn eight_connected<State, N, E, NV, EV, NF, EF>(
        node_val: NV,
        edge_val: EV,
        node_filter: NF,
        edge_filter: EF,
    ) -> StateGraph<State, N, E, NV, EV, impl Fn(State) -> Vec<State>, impl Fn(State) -> bool>
    where
        State: Eq + Hash + Copy + Debug + Add<Direction, Output = State>,
        NV: Fn(State) -> N,
        EV: Fn((State, State)) -> E,
        NF: Fn(State) -> bool,
        EF: Fn(State, State) -> bool,
    {
        StateGraph::new(
            node_val,
            edge_val,
            move |state: State| {
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
                    let adj = state + direction;
                    if (edge_filter)(state, adj) {
                        neighbors.push(adj);
                    }
                }
                neighbors
            },
            node_filter,
        )
    }
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

pub struct GridBounds {
    i: Option<Range<isize>>,
    j: Option<Range<isize>>,
}

impl GridBounds {
    pub fn new(i: Range<isize>, j: Range<isize>) -> GridBounds {
        GridBounds {
            i: Some(i),
            j: Some(j),
        }
    }

    pub fn unbounded() -> GridBounds {
        GridBounds { i: None, j: None }
    }

    pub fn check(&self, p: Position) -> bool {
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

    pub fn len(&self) -> (usize, usize) {
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

    pub fn iter(&self) -> Product<Range<isize>, Range<isize>> {
        let irange = self.i.clone().unwrap_or(isize::MIN..isize::MAX);
        let jrange = self.j.clone().unwrap_or(isize::MIN..isize::MAX);
        irange.cartesian_product(jrange)
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::bfs::bfs;

    use super::{GridBounds, Grid, Position};

    #[test]
    fn four_connected_bfs() {
        let bounds = GridBounds::new(-1..2, -1..2);
        let g = Grid::four_connected(|_| (), |_| 1, |pos| bounds.check(pos), |_, _| true);
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
