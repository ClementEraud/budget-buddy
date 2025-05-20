use tauri::{command, State};

use crate::{domain::entities::operation::Operation, shared::types::array::Array, Queries};

#[command]
pub fn get_balance(queries: State<'_, Queries>) -> i32 {
    queries.get_balance_query.get_balance()
}

#[command]
pub fn get_incomes(queries: State<'_, Queries>) -> Array<Operation> {
    queries.get_incomes_query.get_incomes()
}

#[command]
pub fn get_expenses(queries: State<'_, Queries>) -> Array<Operation> {
    queries.get_expenses_query.get_expenses()
}
