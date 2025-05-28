use std::slice::Iter;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Array<T: Serialize> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T: Serialize> Array<T> {
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { items: vec }
    }

    pub fn iter(&self) -> Iter<T> {
        self.items.iter()
    }

    pub fn filter<F>(&self, predicate: F) -> Array<T>
    where
        T: Clone,
        F: Fn(&T) -> bool,
    {
        Array::from_vec(
            self.items
                .iter()
                .filter(|item| predicate(item))
                .cloned()
                .collect(),
        )
    }

    pub fn map<F, U>(&self, mapper: F) -> Array<U>
    where
        F: Fn(&T) -> U,
        U: Serialize,
    {
        Array::from_vec(self.items.iter().map(|item| mapper(item)).collect())
    }

    pub fn reduce<F, U>(&self, initial: U, reducer: F) -> U
    where
        F: Fn(&T, &U) -> U,
    {
        self.items
            .iter()
            .fold(initial, |acc, item| reducer(item, &acc))
    }

    pub fn length(&self) -> usize {
        self.items.len()
    }
}
