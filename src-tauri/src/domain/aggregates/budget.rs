use crate::{
    domain::{entities::operation::Operation, traits::financial_aggregate::FinancialAggregate},
    shared::types::array::Array,
};

#[derive(Debug)]
pub struct Budget {
    #[allow(dead_code)]
    id: String,
    operations: Array<Operation>,
}

impl Budget {
    #[allow(dead_code)]
    pub fn new(id: String, operations: Array<Operation>) -> Self {
        Self { id, operations }
    }
}

impl FinancialAggregate for Budget {
    fn get_operations(&self) -> Array<Operation> {
        self.operations.clone()
    }
}
