use serde::Serialize;

use crate::shared::{entities::operation::Operation, types::array::Array};

#[derive(Clone, Serialize, Debug)]
pub struct Summary {
    pub incomes: Array<Operation>,
    pub expenses: Array<Operation>,
    pub total_income: f64,
    pub total_expense: f64,
    pub balance: f64,
}
