use crate::domain::aggregates::account::Account;

pub trait AccountQueryRepositoryPort {
    fn get_account(&self) -> Account;
}
