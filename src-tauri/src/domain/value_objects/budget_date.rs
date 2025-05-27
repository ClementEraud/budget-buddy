use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetDate {
    year: u16,
    month: u8,
}
