use tauri::{command, State};

use crate::{shared::value_objects::summary::Summary, Queries};

#[command]
pub fn get_account_summary(queries: State<'_, Queries>) -> Summary {
    queries.get_account_summary.execute()
}
