use crate::{
    application::ports::queries::account::AccountQueryRepositoryPort,
    domain::{traits::financial_aggregate::FinancialAggregate, value_objects::summary::Summary},
};

pub struct GetAccountSummaryQuery<T: AccountQueryRepositoryPort> {
    account_query_repository: T,
}

impl<T: AccountQueryRepositoryPort> GetAccountSummaryQuery<T> {
    pub fn new(account_query_repository: T) -> Self {
        Self {
            account_query_repository,
        }
    }

    pub fn get_account_summary(&self) -> Summary {
        let account = &self.account_query_repository.get_account();
        account.get_summary()
    }
}
