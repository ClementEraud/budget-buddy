use crate::{
    application::ports::queries::operations::OperationsQueryRepositoryPort,
    domain::{array::Array, income::Income},
};

pub struct GetIncomesQuery<T: OperationsQueryRepositoryPort> {
    operations_query_repository: T,
}

impl<T: OperationsQueryRepositoryPort> GetIncomesQuery<T> {
    pub fn new(operations_query_repository: T) -> Self {
        Self {
            operations_query_repository,
        }
    }

    pub fn get_incomes(&self) -> Array<Income> {
        let operations = &self.operations_query_repository.get_operations();
        operations.get_incomes().clone()
    }
}
