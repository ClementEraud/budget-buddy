use crate::{
    queries::{
        application::ports::account_repository::AccountRepositoryPort,
        domain::traits::financial_aggregate::FinancialAggregate,
    },
    shared::value_objects::{budget_date::BudgetDate, summary::Summary},
};

pub struct GetAccountSummaryForDate<T: AccountRepositoryPort> {
    account_query_repository: T,
}

impl<T: AccountRepositoryPort> GetAccountSummaryForDate<T> {
    pub fn new(account_query_repository: T) -> Self {
        Self {
            account_query_repository,
        }
    }

    pub fn execute(&self, date: &BudgetDate) -> Option<Summary> {
        match &self.account_query_repository.get_account_for_date(date) {
            Some(account) => Some(account.get_summary()),
            None => None,
        }
    }
}
