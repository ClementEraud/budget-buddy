use tauri::{command, State};

use crate::{
    commands::application::errors::create_budget_error::CreateBudgetError,
    shared::{
        entities::operation::Operation, types::array::Array, value_objects::budget_date::BudgetDate,
    },
    Commands,
};

#[command]
pub fn create_budget(
    budget_date: BudgetDate,
    operations: Array<Operation>,
    budget_commands: State<'_, Commands>,
) -> Result<(), CreateBudgetError> {
    budget_commands
        .create_budget
        .execute(budget_date, operations)
}
