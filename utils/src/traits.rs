use std::collections::HashMap;

pub trait Updateable<K, V> {
    fn update(&mut self, key: K, delta: V);
}

impl<K, V> Updateable<K, V> for HashMap<K, V> where
        K: std::cmp::Eq + std::hash::Hash,
        V: std::default::Default + std::ops::AddAssign {
    fn update(&mut self, key: K, delta: V) {
        *self.entry(key).or_default() += delta;
    }
}

pub trait ArgMax<T> {
    fn argmax(self) -> Option<usize>;
}

impl<T, Iter> ArgMax<T> for Iter where
        T: std::cmp::Ord,
        Iter: Iterator<Item=T> {
    fn argmax(self) -> Option<usize> {
        // select_fold1 from iterator.rs
        let mut it = self.enumerate();
        it.next().map(|first| {
            it.fold(first, |(i_sel, x_sel), (i_val, x_val)| {
                if x_val >= x_sel { (i_val, x_val) } else { (i_sel, x_sel) }
            })
        }).map(|(i, _)| i)
    }
}
