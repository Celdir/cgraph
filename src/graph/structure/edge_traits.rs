pub trait Directed {}
pub trait Undirected {}

pub trait Weighted<W> {
    fn weight(&self) -> W;
    fn set_weight(&mut self, val: W);
}

pub trait Capacity<C> {
    fn capacity(&self) -> C;
    fn set_capacity(&mut self, val: C);
}
