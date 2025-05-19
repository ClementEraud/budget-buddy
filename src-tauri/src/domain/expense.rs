use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Expense {
    label: String,
    value: i32,
}

impl Expense {
    pub fn new(label: String, value: i32) -> Self {
        Self { label, value }
    }

    pub fn get_value(&self) -> &i32 {
        &self.value
    }
}
