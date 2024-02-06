use std::cell::Cell;
use std::cmp::Ordering;
use std::ops::AddAssign;

#[derive(Debug, Clone)]
pub struct DisjointSet {
    parents: Vec<Cell<usize>>,
    ranks: Vec<u8>,
}

impl Default for DisjointSet {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DisjointSet {
    pub fn new() -> Self {
        Self {
            parents: Vec::new(),
            ranks: Vec::new(),
        }
    }

    pub fn with_len(len: usize) -> Self {
        Self {
            parents: (0..len).map(Cell::new).collect(),
            ranks: vec![0; len],
        }
    }

    pub fn len(&self) -> usize {
        self.parents.len()
    }

    pub fn push_set(&mut self) {
        self.parents.push(Cell::new(self.len()));
        self.ranks.push(0);
    }

    pub fn root(&self, mut child: usize) -> usize {
        let mut parent = self.get_parent(child);
        while parent != child {
            let gp = self.get_parent(parent);
            self.set_parent(child, gp);
            child = parent;
            parent = gp;
        }
        parent
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let a_root = self.root(a);
        let b_root = self.root(b);
        if a_root == b_root {
            return false;
        }

        let b_rank = self.ranks[b_root];
        let a_rank = self.ranks.get_mut(a_root).unwrap();
        match (*a_rank).cmp(&b_rank) {
            Ordering::Less => {
                self.set_parent(a_root, b_root);
            }
            Ordering::Greater => {
                self.set_parent(b_root, a_root);
            }
            Ordering::Equal => {
                *a_rank += 1;
                self.set_parent(b_root, a_root);
            }
        };
        true
    }

    pub fn same_set(&self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    fn get_parent(&self, id: usize) -> usize {
        self.parents[id].get()
    }

    fn set_parent(&self, id: usize, p: usize) {
        self.parents[id].set(p)
    }
}

#[derive(Debug, Clone)]
pub struct WeightedDisjointSet<W> {
    ds: DisjointSet,
    weight: Vec<W>,
}

impl<W> Default for WeightedDisjointSet<W>
where
    W: Default + Clone + AddAssign,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<W> WeightedDisjointSet<W>
where
    W: Default + Clone + AddAssign,
{
    pub fn new() -> Self {
        Self {
            ds: DisjointSet::new(),
            weight: Vec::new(),
        }
    }

    pub fn with_len(len: usize) -> Self {
        Self {
            ds: DisjointSet::with_len(len),
            weight: vec![W::default(); len],
        }
    }

    pub fn len(&self) -> usize {
        self.ds.len()
    }

    pub fn push_set(&mut self) {
        self.ds.push_set();
        self.weight.push(W::default());
    }

    pub fn root(&self, child: usize) -> usize {
        self.ds.root(child)
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let a_root = self.root(a);
        let b_root = self.root(b);
        let changed = self.ds.union(a, b);
        if changed {
            if self.root(a_root) == b_root {
                let delta = self.weight[a_root].clone();
                self.weight[b_root] += delta;
            } else {
                let delta = self.weight[b_root].clone();
                self.weight[a_root] += delta;
            }
        }
        changed
    }

    pub fn same_set(&self, a: usize, b: usize) -> bool {
        self.ds.same_set(a, b)
    }

    pub fn weight(&self, id: usize) -> &W {
        &self.weight[self.root(id)]
    }
}
