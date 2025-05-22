use crate::{
    application::ports::queries::account::AccountQueryRepositoryPort,
    domain::entities::operation::Operation, shared::types::array::Array,
};

pub struct GetAccountIncomesQuery<T: AccountQueryRepositoryPort> {
    account_query_repository: T,
}

impl<T: AccountQueryRepositoryPort> GetAccountIncomesQuery<T> {
    pub fn new(account_query_repository: T) -> Self {
        Self {
            account_query_repository,
        }
    }

    pub fn get_incomes(&self) -> Array<Operation> {
        let account = &self.account_query_repository.get_account();
        account.get_incomes().clone()
    }
}
