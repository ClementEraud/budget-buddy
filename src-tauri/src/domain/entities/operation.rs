use crate::domain::value_objects::operation_type::OperationType;
use serde::Serialize;

#[derive(Clone, Serialize, Debug)]
pub struct Operation {
    pub id: String,
    pub label: String,
    pub value: i32,
    pub operation_type: OperationType,
}

impl Operation {
    pub fn new(id: String, label: String, value: i32, operation_type: OperationType) -> Self {
        Self {
            id,
            label,
            value,
            operation_type,
        }
    }

    pub fn get_value(&self) -> &i32 {
        &self.value
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
