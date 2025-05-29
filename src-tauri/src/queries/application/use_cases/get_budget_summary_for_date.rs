use crate::{
    queries::{
        application::ports::budget_repository::BudgetRepositoryPort,
        domain::traits::financial_aggregate::FinancialAggregate,
    },
    shared::value_objects::{budget_date::BudgetDate, summary::Summary},
};

pub struct GetBudgetSummaryForDate<T: BudgetRepositoryPort> {
    budget_repository: T,
}

impl<T: BudgetRepositoryPort> GetBudgetSummaryForDate<T> {
    pub fn new(budget_repository: T) -> Self {
        Self { budget_repository }
    }

    pub fn execute(&self, date: &BudgetDate) -> Option<Summary> {
        match self.budget_repository.get_budget_for_date(date) {
            Ok(budget) => match budget {
                Some(budget) => Some(budget.get_summary()),
                None => None,
            },
            Err(error) => {
                eprintln!("Error getting budget for date: {}", error);
                None
            }
        }
    }
}
