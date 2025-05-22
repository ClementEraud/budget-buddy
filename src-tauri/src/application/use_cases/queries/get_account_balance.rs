use crate::application::ports::queries::account::AccountQueryRepositoryPort;

pub struct GetAccountBalanceQuery<T: AccountQueryRepositoryPort> {
    account_query_repository: T,
}

impl<T: AccountQueryRepositoryPort> GetAccountBalanceQuery<T> {
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
