use crate::{
    domain::{
        entities::operation::Operation, traits::financial_aggregate::FinancialAggregateQuery,
    },
    shared::types::array::Array,
};

#[derive(Debug)]
pub struct AccountQuery {
    #[allow(dead_code)]
    id: String,
    operations: Array<Operation>,
}

impl AccountQuery {
    pub fn new(id: String, operations: Array<Operation>) -> Self {
        Self { id, operations }
    }
}

impl FinancialAggregateQuery for AccountQuery {
    fn get_operations(&self) -> Array<Operation> {
        self.operations.clone()
    }
}
