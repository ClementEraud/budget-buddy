use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum IncomeType {
    Salary,
    Bonus,
    Other,
}
