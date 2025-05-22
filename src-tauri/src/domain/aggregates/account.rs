use crate::{
    domain::{entities::operation::Operation, traits::financial_aggregate::FinancialAggregate},
    shared::types::array::Array,
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
