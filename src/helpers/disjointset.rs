use std::{cmp::Reverse, collections::HashSet, fmt::Debug, hash::Hash};

#[derive(Debug)]
pub struct DisjointSet<T: Debug + Eq + Hash + Copy> {
    sets: Vec<HashSet<T>>,
}

impl<T: Debug + Eq + Hash + Copy> DisjointSet<T> {
    pub fn new() -> Self {
        DisjointSet { sets: Vec::new() }
    }

    pub fn insert(&mut self, x: &T, y: &T) {
        let mut candidates = self
            .sets
            .iter_mut()
            .enumerate()
            .filter(|(_, c)| c.contains(x) || c.contains(y));

        let c1 = candidates.next();
        if c1.is_none() {
            self.sets.push(HashSet::from([*x, *y]));
            return;
        }

        let (_, c1) = c1.unwrap();
        if c1.contains(x) && c1.contains(y) {
            return;
        }

        let c2 = candidates.next();
        if c2.is_none() {
            c1.insert(*x);
            c1.insert(*y);
            return;
        }

        let (idx2, c2) = c2.unwrap();
        c1.extend(c2.iter());
        self.sets.remove(idx2);
    }

    pub fn iter(&self) -> impl Iterator<Item = &HashSet<T>> {
        self.sets.iter()
    }

    pub fn sort(&mut self) {
        self.sets.sort_by_key(|x| Reverse(x.len()));
    }

    pub fn len(&self) -> usize {
        self.sets.len()
    }
}
