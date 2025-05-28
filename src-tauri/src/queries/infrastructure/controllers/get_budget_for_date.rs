use tauri::{command, State};

use crate::{
    queries::domain::aggregates::budget::Budget, shared::value_objects::budget_date::BudgetDate,
    Queries,
};

#[command]
pub fn get_budget_for_date(date: BudgetDate, queries: State<'_, Queries>) -> Option<Budget> {
    queries.get_budget_for_date.execute(&date)
}
