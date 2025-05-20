use crate::application::ports::queries::account::AccountQueryRepositoryPort;

pub struct GetBalanceQuery<T: AccountQueryRepositoryPort> {
    account_query_repository: T,
}

impl<T: AccountQueryRepositoryPort> GetBalanceQuery<T> {
    pub fn new(account_query_repository: T) -> Self {
        Self {
            account_query_repository,
        }
    }

    pub fn get_balance(&self) -> i32 {
        let account = &self.account_query_repository.get_account();
        account.get_balance()
    }
}
