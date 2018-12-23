use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Val<T: Ord>(T);

impl<T: Ord> Ord for Val<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl<T: Ord> PartialOrd for Val<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct MinHeap<T: Ord> {
    heap: BinaryHeap<Val<T>>
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap { heap: BinaryHeap::<Val<T>>::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn push(&mut self, item: T) {
        self.heap.push(Val(item));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|Val(x)| x)
    }
}
