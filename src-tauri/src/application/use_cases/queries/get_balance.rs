use crate::application::ports::queries::operations::OperationsQueryRepositoryPort;

pub struct GetBalanceQuery<T: OperationsQueryRepositoryPort> {
    operations_query_repository: T,
}

impl<T: OperationsQueryRepositoryPort> GetBalanceQuery<T> {
    pub fn new(operations_query_repository: T) -> Self {
        Self {
            operations_query_repository,
        }
    }

    pub fn get_balance(&self) -> i32 {
        let operations = &self.operations_query_repository.get_operations();
        operations.get_balance()
    }
}
