use crate::queries::domain::aggregates::account::Account;

pub trait AccountRepositoryPort {
    fn get_account(&self) -> Account;
}
