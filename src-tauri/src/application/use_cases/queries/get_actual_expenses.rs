use crate::{
    application::ports::queries::operations::OperationsQueryRepositoryPort,
    domain::{array::Array, expense::Expense},
};

pub struct GetActualExpensesQuery<T: OperationsQueryRepositoryPort> {
    operations_query_repository: T,
}

impl<T: OperationsQueryRepositoryPort> GetActualExpensesQuery<T> {
    pub fn new(operations_query_repository: T) -> Self {
        Self {
            operations_query_repository,
        }
    }

    pub fn get_actual_expenses(&self) -> Array<Expense> {
        let operations = &self.operations_query_repository.get_operations();
        operations.get_actual_expenses().clone()
    }
}
