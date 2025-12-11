use std::{collections::HashMap, fmt::Debug, hash::Hash};

#[derive(Debug)]
pub struct DirectedAcyclicGraph<T: Debug + Eq + PartialEq + Hash> {
    nodes: HashMap<T, Vec<T>>,
}

impl<T: Debug + Eq + PartialEq + Hash> DirectedAcyclicGraph<T> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn insert(&mut self, value: T, children: Vec<T>) {
        self.nodes.insert(value, children);
    }

    pub fn get_children(&self, value: &T) -> Option<&[T]> {
        match self.nodes.get(value) {
            Some(x) => Some(x),
            None => None,
        }
    }

    pub fn count_paths(&self, start: &T, end: &T) -> usize {
        let mut queue: Vec<&T> = Vec::from(&[start]);
        let mut res = 0;
        while !queue.is_empty() {
            let x = queue.pop().unwrap();
            if x == end {
                res += 1;
            } else {
                for c in self.get_children(x).unwrap().iter() {
                    queue.push(c);
                }
            }
        }

        res
    }
}
