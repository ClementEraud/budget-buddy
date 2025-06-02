use crate::{
    queries::domain::traits::financial_aggregate::FinancialAggregate,
    shared::{
        entities::operation::Operation, types::array::Array, value_objects::budget_date::BudgetDate,
    },
};

#[derive(Debug)]
pub struct Account {
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    date: BudgetDate,
    operations: Array<Operation>,
}

impl Account {
    pub fn new(id: String, date: BudgetDate, operations: Array<Operation>) -> Self {
        Self {
            id,
            date,
            operations,
        }
    }
}

impl FinancialAggregate for Account {
    fn get_operations(&self) -> Array<Operation> {
        self.operations.clone()
    }
}
