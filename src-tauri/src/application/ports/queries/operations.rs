use crate::domain::operations::Operations;

pub trait OperationsQueryRepositoryPort {
    fn get_operations(&self) -> Operations;
}
