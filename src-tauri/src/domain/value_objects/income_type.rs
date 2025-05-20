use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
#[allow(dead_code)]
pub enum IncomeType {
    Salary,
    Bonus,
    Other,
}
