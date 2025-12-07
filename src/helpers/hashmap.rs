use std::{collections::HashMap, hash::Hash};

pub trait CountMap<T: Hash + Eq> {
    fn insert_or_increment(&mut self, k: T, v: usize);
}

impl<T: Hash + Eq> CountMap<T> for HashMap<T, usize> {
    fn insert_or_increment(&mut self, k: T, v: usize) {
        let x = self.entry(k).or_insert(0);
        *x += v;
    }
}
