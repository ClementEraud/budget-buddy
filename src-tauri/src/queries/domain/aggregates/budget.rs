use serde::{Deserialize, Serialize};

use crate::{
    queries::domain::traits::financial_aggregate::FinancialAggregate,
    shared::{
        entities::operation::Operation, types::array::Array, value_objects::budget_date::BudgetDate,
    },
};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Budget {
    id: String,
    date: BudgetDate,
    operations: Array<Operation>,
}

impl Budget {
    pub fn date(&self) -> &BudgetDate {
        &self.date
    }
}

impl FinancialAggregate for Budget {
    fn get_operations(&self) -> Array<Operation> {
        self.operations.clone()
    }
}
