use crate::{
    application::ports::queries::operations::OperationsQueryRepositoryPort,
    domain::{array::Array, expense::Expense},
};

pub struct GetPlannedExpensesQuery<T: OperationsQueryRepositoryPort> {
    operations_query_repository: T,
}

impl<T: OperationsQueryRepositoryPort> GetPlannedExpensesQuery<T> {
    pub fn new(operations_query_repository: T) -> Self {
        Self {
            operations_query_repository,
        }
    }

    pub fn get_planned_expenses(&self) -> Array<Expense> {
        let operations = &self.operations_query_repository.get_operations();
        operations.get_planned_expenses().clone()
    }
}
