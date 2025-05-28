use serde::{Deserialize, Serialize};

use crate::shared::{
    entities::operation::Operation, types::array::Array, value_objects::budget_date::BudgetDate,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Budget {
    id: String,
    date: BudgetDate,
    operations: Array<Operation>,
}

impl Budget {
    pub fn new(id: String, date: BudgetDate, operations: Array<Operation>) -> Self {
        Self {
            id,
            date,
            operations,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
