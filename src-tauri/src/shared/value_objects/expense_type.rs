use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum ExpenseType {
    Food,
    Transportation,
    Housing,
    Utilities,
    Entertainment,
    Shopping,
    Health,
    Education,
    Savings,
    Communication,
    Other,
}
