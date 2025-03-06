use crate::{
    application::ports::queries::operations::OperationsQueryRepositoryPort,
    domain::{array::Array, expense::Expense, income::Income, operations::Operations},
};

pub struct OperationsQueryRepository {}

impl OperationsQueryRepositoryPort for OperationsQueryRepository {
    fn get_operations(&self) -> Operations {
        Operations::new(
            Array::from_vec(vec![
                Income::new(String::from("Salary"), 1600),
                Income::new(String::from("Social Aids"), 400),
            ]),
            Array::from_vec(vec![
                Expense::new(String::from("Rent"), 600),
                Expense::new(String::from("Netflix"), 20),
                Expense::new(String::from("Internet"), 50),
                Expense::new(String::from("Mobile phone"), 50),
                Expense::new(String::from("Electricity"), 80),
                Expense::new(String::from("Other bills"), 400),
            ]),
            Array::from_vec(vec![Expense::new(String::from("Groceries"), 300)]),
        )
    }
}

impl OperationsQueryRepository {
    pub fn new() -> Self {
        Self {}
    }
}
