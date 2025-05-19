use std::slice::Iter;

use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Array<T: Serialize> {
    items: Vec<T>,
}

impl<T: Serialize> Array<T> {
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self { items: vec }
    }

    pub fn iter(&self) -> Iter<T> {
        self.items.iter()
    }
}
