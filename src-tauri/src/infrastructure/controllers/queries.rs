use tauri::command;

use crate::{
    application::use_cases::queries::{
        get_actual_expenses::GetActualExpensesQuery, get_balance::GetBalanceQuery,
        get_incomes::GetIncomesQuery, get_planned_expenses::GetPlannedExpensesQuery,
    },
    domain::{array::Array, expense::Expense, income::Income},
    infrastructure::adapters::queries::operations::OperationsQueryRepository,
};

#[command]
pub fn get_balance() -> i32 {
    let operations_query_repository = OperationsQueryRepository::new();
    GetBalanceQuery::new(operations_query_repository).get_balance()
}

#[command]
pub fn get_incomes() -> Array<Income> {
    let operations_query_repository = OperationsQueryRepository::new();
    GetIncomesQuery::new(operations_query_repository).get_incomes()
}

#[command]
pub fn get_planned_expenses() -> Array<Expense> {
    let operations_query_repository = OperationsQueryRepository::new();
    GetPlannedExpensesQuery::new(operations_query_repository).get_planned_expenses()
}

#[command]
pub fn get_actual_expenses() -> Array<Expense> {
    let operations_query_repository = OperationsQueryRepository::new();
    GetActualExpensesQuery::new(operations_query_repository).get_actual_expenses()
}
