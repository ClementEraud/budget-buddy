use crate::commands::{
    domain::aggregates::budget::Budget,
    infrastructure::adapters::errors::save_budget_error::SaveBudgetError,
};

pub trait BudgetRepositoryPort {
    fn create_budget(&self, budget: Budget) -> Result<(), SaveBudgetError>;
}
