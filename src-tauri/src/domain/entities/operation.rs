use crate::domain::value_objects::operation_type::OperationType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Operation {
    pub id: String,
    pub label: String,
    pub amount: f64,
    pub operation_type: OperationType,
}

impl Operation {
    pub fn new(id: String, label: String, amount: f64, operation_type: OperationType) -> Self {
        Self {
            id,
            label,
            amount,
            operation_type,
        }
    }

    pub fn get_amount(&self) -> &f64 {
        &self.amount
    }

    pub fn is_income(&self) -> bool {
        match &self.operation_type {
            OperationType::Income(_) => true,
            _ => false,
        }
    }

    pub fn is_expense(&self) -> bool {
        match &self.operation_type {
            OperationType::Expense(_) => true,
            _ => false,
        }
    }
}
