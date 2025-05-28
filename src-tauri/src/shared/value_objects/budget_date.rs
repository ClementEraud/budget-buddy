use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BudgetDate {
    pub year: u16,
    pub month: u8,
}
