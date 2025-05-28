use crate::{
    queries::{
        domain::aggregates::budget::Budget,
        infrastructure::adapters::errors::get_budget_error::GetBudgetError,
    },
    shared::value_objects::budget_date::BudgetDate,
};

pub trait BudgetRepositoryPort {
    fn get_budget_for_date(&self, date: &BudgetDate) -> Result<Option<Budget>, GetBudgetError>;
}
