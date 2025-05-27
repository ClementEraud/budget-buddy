use uuid::Uuid;

use crate::{
    application::{
        errors::create_budget_error::CreateBudgetError,
        ports::commands::budget::BudgetCommandRepositoryPort,
    },
    domain::{
        aggregates::commands::budget::BudgetCommand, entities::operation::Operation,
        value_objects::budget_date::BudgetDate,
    },
    shared::types::array::Array,
};

pub struct CreateBudgetCommand<T: BudgetCommandRepositoryPort> {
    budget_command_repository: T,
}

impl<T: BudgetCommandRepositoryPort> CreateBudgetCommand<T> {
    pub fn new(budget_command_repository: T) -> Self {
        Self {
            budget_command_repository,
        }
    }

    pub fn create_budget(
        &self,
        budget_date: BudgetDate,
        operations: Array<Operation>,
    ) -> Result<(), CreateBudgetError> {
        let budget_id = Uuid::new_v4();
        let budget = BudgetCommand::new(budget_id.to_string(), budget_date, operations);

        self.budget_command_repository
            .create_budget(budget)
            .map_err(CreateBudgetError::from)
    }
}
