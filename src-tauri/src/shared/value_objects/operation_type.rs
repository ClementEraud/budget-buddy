use serde::{Deserialize, Serialize};

use super::{expense_type::ExpenseType, income_type::IncomeType};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum OperationType {
    Expense(ExpenseType),
    Income(IncomeType),
}
