use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub enum IncomeType {
    Salary,
    Bonus,
    Other,
}
