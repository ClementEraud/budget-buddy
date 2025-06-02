use crate::{
    queries::domain::aggregates::account::Account, shared::value_objects::budget_date::BudgetDate,
};

pub trait AccountRepositoryPort {
    fn get_account_for_date(&self, date: &BudgetDate) -> Option<Account>;
}
