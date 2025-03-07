use tauri::{command, State};

use crate::{
    domain::{array::Array, expense::Expense, income::Income},
    Queries,
};

#[command]
pub fn get_balance(queries: State<'_, Queries>) -> i32 {
    queries.get_balance_query.get_balance()
}

#[command]
pub fn get_incomes(queries: State<'_, Queries>) -> Array<Income> {
    queries.get_incomes_query.get_incomes()
}

#[command]
pub fn get_planned_expenses(queries: State<'_, Queries>) -> Array<Expense> {
    queries.get_planned_expenses_query.get_planned_expenses()
}

#[command]
pub fn get_actual_expenses(queries: State<'_, Queries>) -> Array<Expense> {
    queries.get_actual_expenses_query.get_actual_expenses()
}
