use crate::{
    domain::aggregates::commands::budget::BudgetCommand,
    infrastructure::adapters::errors::save_budget_error::SaveBudgetError,
};

pub trait BudgetCommandRepositoryPort {
    fn create_budget(&self, budget: BudgetCommand) -> Result<(), SaveBudgetError>;
}
