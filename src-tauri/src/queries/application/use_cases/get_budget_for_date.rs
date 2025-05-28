use crate::{
    queries::{
        application::ports::budget_repository::BudgetRepositoryPort,
        domain::aggregates::budget::Budget,
    },
    shared::value_objects::budget_date::BudgetDate,
};

pub struct GetBudgetForDate<T: BudgetRepositoryPort> {
    budget_repository: T,
}

impl<T: BudgetRepositoryPort> GetBudgetForDate<T> {
    pub fn new(budget_repository: T) -> Self {
        Self { budget_repository }
    }

    pub fn execute(&self, date: &BudgetDate) -> Option<Budget> {
        match self.budget_repository.get_budget_for_date(date) {
            Ok(budget) => budget,
            Err(error) => {
                eprintln!("Error getting budget for date: {}", error);
                None
            }
        }
    }
}
