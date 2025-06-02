use tauri::{command, State};

use crate::{
    shared::value_objects::{budget_date::BudgetDate, summary::Summary},
    Queries,
};

#[command]
pub fn get_account_summary_for_date(
    queries: State<'_, Queries>,
    date: BudgetDate,
) -> Option<Summary> {
    queries.get_account_summary_for_date.execute(&date)
}
