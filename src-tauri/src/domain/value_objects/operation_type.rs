use serde::Serialize;

use super::{expense_type::ExpenseType, income_type::IncomeType};

#[derive(Clone, Serialize, Debug)]
pub enum OperationType {
    Expense(ExpenseType),
    Income(IncomeType),
}
