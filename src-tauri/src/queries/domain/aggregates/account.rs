use crate::{
    queries::domain::traits::financial_aggregate::FinancialAggregate,
    shared::{entities::operation::Operation, types::array::Array},
};

#[derive(Debug)]
pub struct Account {
    #[allow(dead_code)]
    id: String,
    operations: Array<Operation>,
}

impl Account {
    pub fn new(id: String, operations: Array<Operation>) -> Self {
        Self { id, operations }
    }
}

impl FinancialAggregate for Account {
    fn get_operations(&self) -> Array<Operation> {
        self.operations.clone()
    }
}
