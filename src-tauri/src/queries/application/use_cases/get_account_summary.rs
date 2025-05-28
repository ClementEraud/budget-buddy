use crate::{
    queries::{
        application::ports::account_repository::AccountRepositoryPort,
        domain::traits::financial_aggregate::FinancialAggregate,
    },
    shared::value_objects::summary::Summary,
};

pub struct GetAccountSummaryQuery<T: AccountRepositoryPort> {
    account_query_repository: T,
}

impl<T: AccountRepositoryPort> GetAccountSummaryQuery<T> {
    pub fn new(account_query_repository: T) -> Self {
        Self {
            account_query_repository,
        }
    }

    pub fn execute(&self) -> Summary {
        let account = &self.account_query_repository.get_account();
        account.get_summary()
    }
}
