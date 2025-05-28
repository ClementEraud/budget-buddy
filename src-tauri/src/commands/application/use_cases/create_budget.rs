use uuid::Uuid;

use crate::{
    commands::{
        application::{
            errors::create_budget_error::CreateBudgetError,
            ports::budget_repository::BudgetRepositoryPort,
        },
        domain::aggregates::budget::Budget,
    },
    shared::{
        entities::operation::Operation, types::array::Array, value_objects::budget_date::BudgetDate,
    },
};

pub struct CreateBudgetCommand<T: BudgetRepositoryPort> {
    budget_repository: T,
}

impl<T: BudgetRepositoryPort> CreateBudgetCommand<T> {
    pub fn new(budget_repository: T) -> Self {
        Self { budget_repository }
    }

    pub fn execute(
        &self,
        budget_date: BudgetDate,
        operations: Array<Operation>,
    ) -> Result<(), CreateBudgetError> {
        let budget_id = Uuid::new_v4();
        let budget = Budget::new(budget_id.to_string(), budget_date, operations);

        self.budget_repository
            .create_budget(budget)
            .map_err(CreateBudgetError::from)
    }
}
