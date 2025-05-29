use tauri::{command, State};

use crate::{
    shared::value_objects::{budget_date::BudgetDate, summary::Summary},
    Queries,
};

#[command]
pub fn get_budget_summary_for_date(
    date: BudgetDate,
    queries: State<'_, Queries>,
) -> Option<Summary> {
    queries.get_budget_summary_for_date.execute(&date)
}
