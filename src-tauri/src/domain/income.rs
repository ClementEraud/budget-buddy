use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Income {
    label: String,
    value: i32,
}

impl Income {
    pub fn new(label: String, value: i32) -> Self {
        Self { label, value }
    }

    pub fn get_value(&self) -> &i32 {
        &self.value
    }
}
