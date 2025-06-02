use crate::{
    queries::{
        application::ports::account_repository::AccountRepositoryPort,
        domain::aggregates::account::Account,
    },
    shared::{
        entities::operation::Operation,
        types::array::Array,
        value_objects::{
            budget_date::BudgetDate, expense_type::ExpenseType, income_type::IncomeType,
            operation_type::OperationType,
        },
    },
};

#[derive(Clone)]
pub struct AccountRepositoryInMemoryAdapter {}

impl AccountRepositoryPort for AccountRepositoryInMemoryAdapter {
    fn get_account_for_date(&self, date: &BudgetDate) -> Option<Account> {
        Some(Account::new(
            String::from("1"),
            date.clone(),
            Array::from_vec(vec![
                Operation::new(
                    String::from("1"),
                    String::from("Salary"),
                    1600.0,
                    OperationType::Income(IncomeType::Salary),
                ),
                Operation::new(
                    String::from("2"),
                    String::from("Social Aids"),
                    400.0,
                    OperationType::Income(IncomeType::Other),
                ),
                Operation::new(
                    String::from("3"),
                    String::from("Rent"),
                    600.0,
                    OperationType::Expense(ExpenseType::Housing),
                ),
                Operation::new(
                    String::from("4"),
                    String::from("Netflix"),
                    20.0,
                    OperationType::Expense(ExpenseType::Entertainment),
                ),
                Operation::new(
                    String::from("5"),
                    String::from("Internet"),
                    50.0,
                    OperationType::Expense(ExpenseType::Entertainment),
                ),
                Operation::new(
                    String::from("6"),
                    String::from("Mobile phone"),
                    50.0,
                    OperationType::Expense(ExpenseType::Communication),
                ),
                Operation::new(
                    String::from("7"),
                    String::from("Electricity"),
                    80.0,
                    OperationType::Expense(ExpenseType::Utilities),
                ),
                Operation::new(
                    String::from("8"),
                    String::from("Other bills"),
                    400.0,
                    OperationType::Expense(ExpenseType::Utilities),
                ),
                Operation::new(
                    String::from("9"),
                    String::from("Groceries"),
                    300.0,
                    OperationType::Expense(ExpenseType::Food),
                ),
            ]),
        ))
    }
}

impl AccountRepositoryInMemoryAdapter {
    pub fn new() -> Self {
        Self {}
    }
}
