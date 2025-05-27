use tauri::{command, State};

use crate::{
    application::errors::create_budget_error::CreateBudgetError,
    domain::{entities::operation::Operation, value_objects::budget_date::BudgetDate},
    shared::types::array::Array,
    Commands,
};

#[command]
pub fn create_budget(
    budget_date: BudgetDate,
    operations: Array<Operation>,
    budget_commands: State<'_, Commands>,
) -> Result<(), CreateBudgetError> {
    budget_commands
        .create_budget_command
        .create_budget(budget_date, operations)
}
