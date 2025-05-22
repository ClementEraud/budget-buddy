use tauri::{command, State};

use crate::{domain::entities::operation::Operation, shared::types::array::Array, Queries};

#[command]
pub fn get_account_balance(queries: State<'_, Queries>) -> i32 {
    queries.get_account_balance_query.get_balance()
}

#[command]
pub fn get_account_incomes(queries: State<'_, Queries>) -> Array<Operation> {
    queries.get_account_incomes_query.get_incomes()
}

#[command]
pub fn get_account_expenses(queries: State<'_, Queries>) -> Array<Operation> {
    queries.get_account_expenses_query.get_expenses()
}
