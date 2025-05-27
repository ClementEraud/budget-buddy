use serde::{Deserialize, Serialize};

use crate::{
    domain::{entities::operation::Operation, value_objects::budget_date::BudgetDate},
    shared::types::array::Array,
};

#[derive(Serialize, Deserialize)]
pub struct BudgetCommand {
    id: String,
    date: BudgetDate,
    operations: Array<Operation>,
}

impl BudgetCommand {
    pub fn new(id: String, date: BudgetDate, operations: Array<Operation>) -> Self {
        Self {
            id,
            date,
            operations,
        }
    }
}
