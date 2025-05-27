use crate::domain::aggregates::queries::account::AccountQuery;

pub trait AccountQueryRepositoryPort {
    fn get_account(&self) -> AccountQuery;
}
