use serde::{Deserialize, Serialize};

use crate::{
    domain::{
        entities::operation::Operation, traits::financial_aggregate::FinancialAggregateQuery,
        value_objects::budget_date::BudgetDate,
    },
    shared::types::array::Array,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Budget {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    date: BudgetDate,
    operations: Array<Operation>,
}

impl Budget {
    #[allow(dead_code)]
    pub fn new(id: String, date: BudgetDate, operations: Array<Operation>) -> Self {
        Self {
            id,
            date,
            operations,
        }
    }
}

impl FinancialAggregateQuery for Budget {
    fn get_operations(&self) -> Array<Operation> {
        self.operations.clone()
    }
}
