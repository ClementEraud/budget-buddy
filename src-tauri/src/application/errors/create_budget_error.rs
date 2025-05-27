use serde::Serialize;

use crate::infrastructure::adapters::errors::save_budget_error::SaveBudgetError;

#[derive(Debug, Clone, Serialize)]
pub struct CreateBudgetError {
    error: String,
}

impl From<SaveBudgetError> for CreateBudgetError {
    fn from(error: SaveBudgetError) -> Self {
        CreateBudgetError {
            error: error.to_string(),
        }
    }
}
